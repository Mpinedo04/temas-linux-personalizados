//! El programa en marcha: un fondo animado por pantalla, en la capa del fondo de COSMIC.

use crate::gfx::Gfx;
use crate::render::{base_scale, kind_of, Fbo, Renderer};
use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState, Region},
    delegate_compositor, delegate_layer, delegate_output, delegate_registry,
    output::{OutputHandler, OutputState},
    reexports::{
        calloop::{
            timer::{TimeoutAction, Timer},
            EventLoop,
        },
        calloop_wayland_source::WaylandSource,
    },
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    shell::{
        wlr_layer::{
            Anchor, KeyboardInteractivity, Layer, LayerShell, LayerShellHandler, LayerSurface,
            LayerSurfaceConfigure,
        },
        WaylandSurface,
    },
};
use std::ffi::c_void;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};
use wayland_client::{
    globals::registry_queue_init,
    protocol::{wl_output, wl_surface},
    Connection, Proxy, QueueHandle,
};
use wayland_egl::WlEglSurface;

const TRANSITION: Duration = Duration::from_millis(1700);
const FLASH: Duration = Duration::from_millis(1100);
const NS_FONDO: &str = "estilos-fondo-vivo";
const NS_DESTELLO: &str = "estilos-destello";

fn home() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").unwrap_or_default())
}

fn read_actual() -> String {
    std::fs::read_to_string(home().join(".local/share/estilos/.actual"))
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

#[derive(Clone, Copy)]
pub struct Config {
    pub fps: u32,
    pub calidad: f32,
    pub destello: bool,
}

impl Config {
    pub fn path() -> PathBuf {
        home().join(".config/estilos/fondos-vivos.conf")
    }

    /// Lee `fps=`, `calidad=` y `destello=` de ~/.config/estilos/fondos-vivos.conf.
    pub fn load() -> Self {
        let mut c = Config { fps: 30, calidad: 1.0, destello: true };
        if let Ok(text) = std::fs::read_to_string(Self::path()) {
            for line in text.lines() {
                let Some((k, v)) = line.split_once('=') else { continue };
                let v = v.trim();
                match k.trim() {
                    "fps" => c.fps = v.parse().unwrap_or(30),
                    "calidad" => {
                        c.calidad = match v {
                            "baja" => 0.6,
                            "media" => 1.0,
                            "alta" => 1.5,
                            _ => v.replace(',', ".").parse().unwrap_or(1.0),
                        }
                    }
                    "destello" => c.destello = !matches!(v, "no" | "0" | "false" | "off"),
                    _ => {}
                }
            }
        }
        c.fps = c.fps.clamp(5, 60);
        c.calidad = c.calidad.clamp(0.25, 2.5);
        c
    }
}

fn mtime(p: &std::path::Path) -> Option<SystemTime> {
    std::fs::metadata(p).and_then(|m| m.modified()).ok()
}

/// Lo que vigilamos para saber si COSMIC ha vuelto a dibujar su fondo encima del nuestro.
#[derive(PartialEq, Default, Clone)]
struct Watch {
    bg_config: Option<SystemTime>,
    bg_pids: Vec<u32>,
}

fn bg_config_mtime() -> Option<SystemTime> {
    let dir = home().join(".config/cosmic/com.system76.CosmicBackground/v1");
    let mut best = mtime(&dir);
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            best = best.max(mtime(&e.path()));
        }
    }
    best
}

fn cosmic_bg_pids() -> Vec<u32> {
    let mut pids = Vec::new();
    if let Ok(rd) = std::fs::read_dir("/proc") {
        for e in rd.flatten() {
            let name = e.file_name();
            let Some(pid) = name.to_str().and_then(|s| s.parse::<u32>().ok()) else { continue };
            if std::fs::read_to_string(e.path().join("comm")).is_ok_and(|c| c.trim() == "cosmic-bg") {
                pids.push(pid);
            }
        }
    }
    pids.sort_unstable();
    pids
}

struct Surf {
    layer: LayerSurface,
    win: Option<WlEglSurface>,
    egl: Option<khronos_egl::Surface>,
    w: i32,
    h: i32,
    scale: i32,
    configured: bool,
    frame_pending: Option<Instant>,
    drawn: bool,
}

impl Surf {
    fn is(&self, s: &wl_surface::WlSurface) -> bool {
        self.layer.wl_surface() == s
    }
}

struct Out {
    output: wl_output::WlOutput,
    bg: Option<Surf>,
    bg_old: Option<Surf>,
    flash: Option<Surf>,
    fbo_cur: Option<Fbo>,
    fbo_prev: Option<Fbo>,
}

pub struct App {
    registry: RegistryState,
    outputs: OutputState,
    compositor: CompositorState,
    layer_shell: LayerShell,
    gfx: Gfx,
    r: Renderer,
    outs: Vec<Out>,
    /// Estilo animado visible; `None` deja ver el fondo normal de COSMIC.
    style: Option<String>,
    actual: String,
    prev: Option<String>,
    trans_start: Option<Instant>,
    flash: Option<(i32, Instant)>,
    start: Instant,
    cfg: Config,
    cfg_mtime: Option<SystemTime>,
    watch: Watch,
    remaps: Vec<Instant>,
    last_poll: Instant,
    last_pid_scan: Instant,
}

fn drop_surf(gfx: &Gfx, s: Surf) {
    if let Some(e) = s.egl {
        gfx.destroy_surface(e);
    }
    drop(s.win);
    drop(s.layer);
}

fn setup_egl(gfx: &Gfx, s: &mut Surf) {
    let (pw, ph) = (s.w * s.scale, s.h * s.scale);
    s.layer.wl_surface().set_buffer_scale(s.scale);
    match &s.win {
        Some(win) => win.resize(pw, ph, 0, 0),
        None => match WlEglSurface::new(s.layer.wl_surface().id(), pw, ph) {
            Ok(win) => {
                s.egl = gfx.create_window_surface(win.ptr());
                s.win = Some(win);
            }
            Err(e) => eprintln!("fondo-vivo: wl_egl_window: {e}"),
        },
    }
}

impl App {
    fn time(&self) -> f32 {
        (self.start.elapsed().as_secs_f64() % 3600.0) as f32
    }

    fn new_surf(&self, qh: &QueueHandle<Self>, output: &wl_output::WlOutput, layer: Layer, ns: &str) -> Surf {
        let surface = self.compositor.create_surface(qh);
        // región de entrada vacía: el ratón y los clics atraviesan la superficie
        if let Ok(region) = Region::new(&self.compositor) {
            surface.set_input_region(Some(region.wl_region()));
        }
        let ls = self.layer_shell.create_layer_surface(qh, surface, layer, Some(ns), Some(output));
        ls.set_anchor(Anchor::TOP | Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT);
        ls.set_exclusive_zone(-1);
        ls.set_keyboard_interactivity(KeyboardInteractivity::None);
        ls.set_size(0, 0);
        ls.commit();
        let scale = self.outputs.info(output).map_or(1, |i| i.scale_factor.max(1));
        Surf { layer: ls, win: None, egl: None, w: 1, h: 1, scale, configured: false, frame_pending: None, drawn: false }
    }

    fn ensure_backgrounds(&mut self, qh: &QueueHandle<Self>) {
        for i in 0..self.outs.len() {
            if self.outs[i].bg.is_none() {
                let s = self.new_surf(qh, &self.outs[i].output.clone(), Layer::Background, NS_FONDO);
                self.outs[i].bg = Some(s);
            }
        }
    }

    fn remove_backgrounds(&mut self) {
        self.gfx.make_current(None);
        for o in &mut self.outs {
            for s in [o.bg.take(), o.bg_old.take()].into_iter().flatten() {
                drop_surf(&self.gfx, s);
            }
            for f in [o.fbo_cur.take(), o.fbo_prev.take()].into_iter().flatten() {
                self.r.delete_fbo(f);
            }
        }
    }

    /// Vuelve a crear nuestras superficies para quedar por encima del fondo de COSMIC.
    /// La antigua se mantiene hasta que la nueva ya tiene imagen: así no hay parpadeo.
    fn remap(&mut self, qh: &QueueHandle<Self>) {
        if self.style.is_none() {
            return;
        }
        for i in 0..self.outs.len() {
            if let Some(old) = self.outs[i].bg_old.take() {
                drop_surf(&self.gfx, old);
            }
            let new = self.new_surf(qh, &self.outs[i].output.clone(), Layer::Background, NS_FONDO);
            self.outs[i].bg_old = self.outs[i].bg.replace(new);
        }
    }

    fn start_flash(&mut self, qh: &QueueHandle<Self>, kind: i32) {
        for i in 0..self.outs.len() {
            if let Some(old) = self.outs[i].flash.take() {
                drop_surf(&self.gfx, old);
            }
            let s = self.new_surf(qh, &self.outs[i].output.clone(), Layer::Overlay, NS_DESTELLO);
            self.outs[i].flash = Some(s);
        }
        self.flash = Some((kind, Instant::now()));
    }

    fn set_style(&mut self, qh: &QueueHandle<Self>, new: String) {
        self.gfx.make_current(None);
        let animated = self.r.has_scene(&new);
        eprintln!("fondo-vivo: estilo «{new}»{}", if animated { "" } else { " (sin fondo animado)" });
        if animated {
            let had = self.style.take();
            if let Some(old) = had.filter(|o| *o != new) {
                self.prev = Some(old);
                self.trans_start = Some(Instant::now());
            }
            self.style = Some(new.clone());
            self.ensure_backgrounds(qh);
        } else {
            self.style = None;
            self.prev = None;
            self.trans_start = None;
            self.remove_backgrounds();
        }
        if self.cfg.destello && kind_of(&new) != 9 {
            self.start_flash(qh, kind_of(&new));
        }
        // COSMIC suele redibujar su fondo al cambiar de estilo: nos recolocamos encima
        let now = Instant::now();
        self.remaps = vec![now + Duration::from_millis(400), now + Duration::from_millis(1600)];
    }

    fn poll(&mut self, qh: &QueueHandle<Self>) {
        let actual = read_actual();
        // «estilo» vacía el archivo y luego lo escribe: una lectura vacía se ignora
        if !actual.is_empty() && actual != self.actual {
            self.actual = actual.clone();
            self.set_style(qh, actual);
        }
        let m = mtime(&Config::path());
        if m != self.cfg_mtime {
            self.cfg_mtime = m;
            self.cfg = Config::load();
            eprintln!("fondo-vivo: ajustes: {} fps, calidad {}, destello {}", self.cfg.fps, self.cfg.calidad, self.cfg.destello);
        }
        let mut w = self.watch.clone();
        w.bg_config = bg_config_mtime();
        if self.last_pid_scan.elapsed() >= Duration::from_secs(1) {
            self.last_pid_scan = Instant::now();
            w.bg_pids = cosmic_bg_pids();
        }
        if w != self.watch {
            self.watch = w;
            let now = Instant::now();
            self.remaps.push(now + Duration::from_millis(250));
            self.remaps.push(now + Duration::from_millis(1500));
        }
    }

    fn tick(&mut self, qh: &QueueHandle<Self>) {
        let now = Instant::now();
        if now.duration_since(self.last_poll) >= Duration::from_millis(250) {
            self.last_poll = now;
            self.poll(qh);
        }
        if self.remaps.iter().any(|t| *t <= now) {
            self.remaps.retain(|t| *t > now);
            self.remap(qh);
        }
        if self.trans_start.is_some_and(|s| s.elapsed() >= TRANSITION) {
            self.trans_start = None;
            self.prev = None;
            self.gfx.make_current(None);
            for o in &mut self.outs {
                if let Some(f) = o.fbo_prev.take() {
                    self.r.delete_fbo(f);
                }
            }
        }
        if self.flash.is_some_and(|(_, s)| s.elapsed() >= FLASH) {
            self.flash = None;
            for o in &mut self.outs {
                if let Some(s) = o.flash.take() {
                    drop_surf(&self.gfx, s);
                }
            }
        }
        let t = self.time();
        for i in 0..self.outs.len() {
            self.render_bg(i, qh, t);
            self.render_flash(i, qh);
            let o = &mut self.outs[i];
            if o.bg.as_ref().is_some_and(|s| s.drawn) {
                if let Some(old) = o.bg_old.take() {
                    drop_surf(&self.gfx, old);
                }
            }
        }
    }

    fn ready(s: &Surf) -> Option<khronos_egl::Surface> {
        if !s.configured {
            return None;
        }
        // si el compositor no avisa (fondo tapado), dibujamos como mucho una vez por segundo
        if s.frame_pending.is_some_and(|t| t.elapsed() < Duration::from_secs(1)) {
            return None;
        }
        s.egl
    }

    fn render_bg(&mut self, i: usize, qh: &QueueHandle<Self>, t: f32) {
        let App { outs, r, gfx, style, prev, trans_start, cfg, .. } = self;
        let Some(style) = style.clone() else { return };
        let o = &mut outs[i];
        let Some(s) = o.bg.as_mut() else { return };
        let Some(es) = Self::ready(s) else { return };
        if !gfx.make_current(Some(es)) {
            return;
        }
        let _ = gfx.egl.swap_interval(gfx.display, 0);
        let (pw, ph) = (s.w * s.scale, s.h * s.scale);
        let dims = |st: &str| {
            let k = (base_scale(st) * cfg.calidad).clamp(0.15, 1.0);
            (((pw as f32) * k) as i32, ((ph as f32) * k) as i32)
        };
        let (cw, ch) = dims(&style);
        r.ensure_fbo(&mut o.fbo_cur, cw, ch);
        r.draw_scene(&style, o.fbo_cur.as_ref().unwrap(), t);
        match (prev.as_ref(), trans_start) {
            (Some(p), Some(ts)) => {
                let (bw, bh) = dims(p);
                r.ensure_fbo(&mut o.fbo_prev, bw, bh);
                r.draw_scene(p, o.fbo_prev.as_ref().unwrap(), t);
                let progress = ts.elapsed().as_secs_f32() / TRANSITION.as_secs_f32();
                r.present_transition(o.fbo_prev.as_ref().unwrap(), o.fbo_cur.as_ref().unwrap(),
                    progress, kind_of(&style), None, pw, ph);
            }
            _ => r.present(o.fbo_cur.as_ref().unwrap(), None, pw, ph),
        }
        let ws = s.layer.wl_surface();
        ws.frame(qh, ws.clone());
        s.frame_pending = Some(Instant::now());
        if gfx.egl.swap_buffers(gfx.display, es).is_ok() {
            s.drawn = true;
        }
    }

    fn render_flash(&mut self, i: usize, qh: &QueueHandle<Self>) {
        let App { outs, r, gfx, flash, .. } = self;
        let Some((kind, start)) = *flash else { return };
        let Some(s) = outs[i].flash.as_mut() else { return };
        let Some(es) = Self::ready(s) else { return };
        if !gfx.make_current(Some(es)) {
            return;
        }
        let _ = gfx.egl.swap_interval(gfx.display, 0);
        let progress = start.elapsed().as_secs_f32() / FLASH.as_secs_f32();
        r.draw_destello(progress, kind, None, s.w * s.scale, s.h * s.scale, true);
        let ws = s.layer.wl_surface();
        ws.frame(qh, ws.clone());
        s.frame_pending = Some(Instant::now());
        let _ = gfx.egl.swap_buffers(gfx.display, es);
    }

    fn surf_mut(&mut self, ws: &wl_surface::WlSurface) -> Option<&mut Surf> {
        self.outs.iter_mut().find_map(|o| {
            [o.bg.as_mut(), o.bg_old.as_mut(), o.flash.as_mut()]
                .into_iter()
                .flatten()
                .find(|s| s.is(ws))
        })
    }
}

impl CompositorHandler for App {
    fn scale_factor_changed(&mut self, _: &Connection, _: &QueueHandle<Self>, ws: &wl_surface::WlSurface, factor: i32) {
        let App { outs, gfx, .. } = self;
        for o in outs.iter_mut() {
            for s in [o.bg.as_mut(), o.bg_old.as_mut(), o.flash.as_mut()].into_iter().flatten() {
                if s.is(ws) && s.scale != factor.max(1) {
                    s.scale = factor.max(1);
                    if s.configured {
                        setup_egl(gfx, s);
                    }
                }
            }
        }
    }
    fn transform_changed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: wl_output::Transform) {}
    fn frame(&mut self, _: &Connection, _: &QueueHandle<Self>, ws: &wl_surface::WlSurface, _: u32) {
        if let Some(s) = self.surf_mut(ws) {
            s.frame_pending = None;
        }
    }
    fn surface_enter(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: &wl_output::WlOutput) {}
    fn surface_leave(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &wl_surface::WlSurface, _: &wl_output::WlOutput) {}
}

impl OutputHandler for App {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.outputs
    }
    fn new_output(&mut self, _: &Connection, qh: &QueueHandle<Self>, output: wl_output::WlOutput) {
        self.outs.push(Out { output, bg: None, bg_old: None, flash: None, fbo_cur: None, fbo_prev: None });
        if self.style.is_some() {
            self.ensure_backgrounds(qh);
        }
        // COSMIC también rehace su fondo al conectar un monitor
        let now = Instant::now();
        self.remaps.push(now + Duration::from_millis(800));
        self.remaps.push(now + Duration::from_millis(2500));
    }
    fn update_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
    fn output_destroyed(&mut self, _: &Connection, _: &QueueHandle<Self>, output: wl_output::WlOutput) {
        if let Some(pos) = self.outs.iter().position(|o| o.output == output) {
            let o = self.outs.remove(pos);
            self.gfx.make_current(None);
            for s in [o.bg, o.bg_old, o.flash].into_iter().flatten() {
                drop_surf(&self.gfx, s);
            }
            for f in [o.fbo_cur, o.fbo_prev].into_iter().flatten() {
                self.r.delete_fbo(f);
            }
        }
    }
}

impl LayerShellHandler for App {
    fn closed(&mut self, _: &Connection, _: &QueueHandle<Self>, layer: &LayerSurface) {
        let ws = layer.wl_surface().clone();
        for o in &mut self.outs {
            for slot in [&mut o.bg, &mut o.bg_old, &mut o.flash] {
                if slot.as_ref().is_some_and(|s| s.is(&ws)) {
                    if let Some(s) = slot.take() {
                        drop_surf(&self.gfx, s);
                    }
                }
            }
        }
    }

    fn configure(&mut self, _: &Connection, _: &QueueHandle<Self>, layer: &LayerSurface, cfg: LayerSurfaceConfigure, _: u32) {
        let ws = layer.wl_surface().clone();
        let App { outs, gfx, .. } = self;
        for o in outs.iter_mut() {
            for s in [o.bg.as_mut(), o.bg_old.as_mut(), o.flash.as_mut()].into_iter().flatten() {
                if s.is(&ws) {
                    s.w = (cfg.new_size.0 as i32).max(1);
                    s.h = (cfg.new_size.1 as i32).max(1);
                    s.configured = true;
                    s.frame_pending = None;
                    setup_egl(gfx, s);
                }
            }
        }
    }
}

impl ProvidesRegistryState for App {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry
    }
    registry_handlers![OutputState];
}

delegate_compositor!(App);
delegate_output!(App);
delegate_layer!(App);
delegate_registry!(App);

pub fn run() -> Result<(), String> {
    let conn = Connection::connect_to_env().map_err(|e| format!("no hay sesión Wayland: {e}"))?;
    let (globals, event_queue) = registry_queue_init::<App>(&conn).map_err(|e| e.to_string())?;
    let qh = event_queue.handle();
    let mut event_loop: EventLoop<App> = EventLoop::try_new().map_err(|e| e.to_string())?;
    WaylandSource::new(conn.clone(), event_queue)
        .insert(event_loop.handle())
        .map_err(|e| e.to_string())?;

    let compositor = CompositorState::bind(&globals, &qh).map_err(|e| format!("wl_compositor: {e}"))?;
    let layer_shell = LayerShell::bind(&globals, &qh)
        .map_err(|e| format!("el escritorio no admite capas (wlr-layer-shell): {e}"))?;
    let gfx = Gfx::new(Some(conn.backend().display_ptr() as *mut c_void))?;
    if !gfx.make_current(None) {
        return Err("la tarjeta gráfica no permite contextos sin superficie".into());
    }
    let r = Renderer::new(gfx.glow())?;
    let cfg = Config::load();

    let mut app = App {
        registry: RegistryState::new(&globals),
        outputs: OutputState::new(&globals, &qh),
        compositor,
        layer_shell,
        gfx,
        r,
        outs: Vec::new(),
        style: None,
        actual: String::new(),
        prev: None,
        trans_start: None,
        flash: None,
        start: Instant::now(),
        cfg,
        cfg_mtime: mtime(&Config::path()),
        watch: Watch { bg_config: bg_config_mtime(), bg_pids: cosmic_bg_pids() },
        remaps: Vec::new(),
        last_poll: Instant::now(),
        last_pid_scan: Instant::now(),
    };
    // estilo inicial, sin destello ni transición
    app.actual = read_actual();
    app.gfx.make_current(None);
    if app.r.has_scene(&app.actual) {
        app.style = Some(app.actual.clone());
    }
    eprintln!("fondo-vivo: en marcha con el estilo «{}»", app.actual);

    let qh2 = qh.clone();
    event_loop
        .handle()
        .insert_source(Timer::immediate(), move |_, _, app: &mut App| {
            app.tick(&qh2);
            TimeoutAction::ToDuration(Duration::from_millis(1000 / app.cfg.fps as u64))
        })
        .map_err(|e| e.to_string())?;

    loop {
        event_loop.dispatch(None, &mut app).map_err(|e| e.to_string())?;
    }
}

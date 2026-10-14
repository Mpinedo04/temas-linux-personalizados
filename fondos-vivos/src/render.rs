//! Compila los shaders y dibuja escenas, transiciones y destellos con OpenGL ES 3.

use glow::HasContext;
use std::collections::HashMap;
use std::path::PathBuf;

const HEADER: &str = "#version 300 es\nprecision highp float;\nprecision highp int;\n";
const COMUN: &str = include_str!("../shaders/comun.glsl");
const TRANSICION: &str = include_str!("../shaders/transicion.frag");
const DESTELLO: &str = include_str!("../shaders/destello.frag");

/// Escenas incluidas en el programa. Se pueden sustituir o añadir otras dejando
/// `<estilo>.frag` en `~/.local/share/estilos/fondos-vivos/`.
pub const ESCENAS: &[(&str, &str)] = &[
    ("hacker", include_str!("../shaders/hacker.frag")),
    ("cyberpunk", include_str!("../shaders/cyberpunk.frag")),
    ("macos", include_str!("../shaders/macos.frag")),
    ("windows", include_str!("../shaders/windows.frag")),
    ("minimalista", include_str!("../shaders/minimalista.frag")),
];

const VERTEX: &str = "#version 300 es
out vec2 vUv;
void main() {
    vec2 p = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2));
    vUv = p;
    gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}
";

const PRESENT: &str = "uniform sampler2D uTex;
in vec2 vUv;
out vec4 fragColor;
void main() { fragColor = vec4(texture(uTex, vUv).rgb, 1.0); }
";

/// Efecto de transición/destello que corresponde a cada estilo.
pub fn kind_of(style: &str) -> i32 {
    match style {
        "hacker" => 0,
        "cyberpunk" => 1,
        "macos" => 2,
        "windows" => 3,
        "minimalista" => 4,
        _ => 9,
    }
}

/// Resolución interna de cada escena (las suaves no necesitan más; ahorra batería).
pub fn base_scale(style: &str) -> f32 {
    match style {
        "hacker" => 0.75,
        "cyberpunk" => 0.6,
        "macos" => 0.4,
        "windows" => 0.5,
        "minimalista" => 0.5,
        _ => 0.6,
    }
}

pub fn user_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share/estilos/fondos-vivos"))
}

/// Código de la escena: primero el del usuario, si existe; si no, el incluido.
pub fn scene_source(style: &str) -> Option<String> {
    if !style.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') || style.is_empty() {
        return None;
    }
    if let Some(dir) = user_dir() {
        if let Ok(s) = std::fs::read_to_string(dir.join(format!("{style}.frag"))) {
            return Some(s);
        }
    }
    ESCENAS.iter().find(|(n, _)| *n == style).map(|(_, s)| s.to_string())
}

pub struct Program {
    id: glow::Program,
    loc: HashMap<&'static str, glow::UniformLocation>,
}

impl Program {
    fn u(&self, name: &'static str) -> Option<&glow::UniformLocation> {
        self.loc.get(name)
    }
}

pub struct Fbo {
    fb: glow::Framebuffer,
    tex: glow::Texture,
    pub w: i32,
    pub h: i32,
}

pub struct Renderer {
    pub gl: glow::Context,
    vao: glow::VertexArray,
    scenes: HashMap<String, Option<Program>>,
    present: Program,
    transition: Program,
    destello: Program,
}

const UNIFORMS: &[&str] = &[
    "iTime", "iResolution", "uTex", "uOld", "uNew", "uProgress", "uKind", "uSize",
];

unsafe fn compile(gl: &glow::Context, frag: &str, what: &str) -> Result<Program, String> {
    let vs = gl.create_shader(glow::VERTEX_SHADER)?;
    gl.shader_source(vs, VERTEX);
    gl.compile_shader(vs);
    if !gl.get_shader_compile_status(vs) {
        return Err(format!("vértices: {}", gl.get_shader_info_log(vs)));
    }
    let fs = gl.create_shader(glow::FRAGMENT_SHADER)?;
    gl.shader_source(fs, frag);
    gl.compile_shader(fs);
    if !gl.get_shader_compile_status(fs) {
        return Err(format!("{what}: {}", gl.get_shader_info_log(fs)));
    }
    let p = gl.create_program()?;
    gl.attach_shader(p, vs);
    gl.attach_shader(p, fs);
    gl.link_program(p);
    gl.delete_shader(vs);
    gl.delete_shader(fs);
    if !gl.get_program_link_status(p) {
        return Err(format!("{what} (enlace): {}", gl.get_program_info_log(p)));
    }
    let mut loc = HashMap::new();
    for name in UNIFORMS {
        if let Some(l) = gl.get_uniform_location(p, name) {
            loc.insert(*name, l);
        }
    }
    Ok(Program { id: p, loc })
}

pub fn scene_fragment(src: &str) -> String {
    format!(
        "{HEADER}uniform float iTime;\nuniform vec2 iResolution;\nout vec4 fragColor;\n{COMUN}\n{src}\n\
         void main() {{ vec4 c = vec4(0.0); mainImage(c, gl_FragCoord.xy); fragColor = vec4(c.rgb, 1.0); }}\n"
    )
}

impl Renderer {
    pub fn new(gl: glow::Context) -> Result<Self, String> {
        unsafe {
            let vao = gl.create_vertex_array()?;
            let present = compile(&gl, &format!("{HEADER}{PRESENT}"), "presentar")?;
            let transition = compile(&gl, &format!("{HEADER}{COMUN}{TRANSICION}"), "transicion.frag")?;
            let destello = compile(&gl, &format!("{HEADER}{COMUN}{DESTELLO}"), "destello.frag")?;
            Ok(Self { gl, vao, scenes: HashMap::new(), present, transition, destello })
        }
    }

    /// ¿Tiene este estilo un fondo animado que compile?
    pub fn has_scene(&mut self, style: &str) -> bool {
        if !self.scenes.contains_key(style) {
            let prog = scene_source(style).and_then(|src| unsafe {
                compile(&self.gl, &scene_fragment(&src), &format!("{style}.frag"))
                    .map_err(|e| eprintln!("fondo-vivo: error en la escena {e}"))
                    .ok()
            });
            self.scenes.insert(style.to_string(), prog);
        }
        self.scenes.get(style).is_some_and(|p| p.is_some())
    }

    pub fn ensure_fbo(&self, slot: &mut Option<Fbo>, w: i32, h: i32) {
        let (w, h) = (w.max(1), h.max(1));
        if slot.as_ref().is_some_and(|f| f.w == w && f.h == h) {
            return;
        }
        if let Some(old) = slot.take() {
            self.delete_fbo(old);
        }
        unsafe {
            let gl = &self.gl;
            let tex = gl.create_texture().expect("textura");
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA8 as i32, w, h, 0, glow::RGBA,
                glow::UNSIGNED_BYTE, glow::PixelUnpackData::Slice(None));
            for (k, v) in [
                (glow::TEXTURE_MIN_FILTER, glow::LINEAR),
                (glow::TEXTURE_MAG_FILTER, glow::LINEAR),
                (glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE),
                (glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE),
            ] {
                gl.tex_parameter_i32(glow::TEXTURE_2D, k, v as i32);
            }
            let fb = gl.create_framebuffer().expect("framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fb));
            gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, Some(tex), 0);
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            *slot = Some(Fbo { fb, tex, w, h });
        }
    }

    pub fn delete_fbo(&self, f: Fbo) {
        unsafe {
            self.gl.delete_framebuffer(f.fb);
            self.gl.delete_texture(f.tex);
        }
    }

    fn draw(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }

    /// Dibuja la escena de un estilo dentro de un framebuffer.
    pub fn draw_scene(&mut self, style: &str, target: &Fbo, time: f32) {
        if !self.has_scene(style) {
            return;
        }
        let Some(Some(p)) = self.scenes.get(style) else { return };
        unsafe {
            let gl = &self.gl;
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(target.fb));
            gl.viewport(0, 0, target.w, target.h);
            gl.disable(glow::BLEND);
            gl.use_program(Some(p.id));
            gl.uniform_1_f32(p.u("iTime"), time);
            gl.uniform_2_f32(p.u("iResolution"), target.w as f32, target.h as f32);
        }
        self.draw();
    }

    /// Copia una escena a la pantalla (`fb` = None) o a otro framebuffer, escalándola.
    pub fn present(&self, src: &Fbo, fb: Option<&Fbo>, w: i32, h: i32) {
        unsafe {
            let gl = &self.gl;
            gl.bind_framebuffer(glow::FRAMEBUFFER, fb.map(|f| f.fb));
            gl.viewport(0, 0, w, h);
            gl.disable(glow::BLEND);
            gl.use_program(Some(self.present.id));
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(src.tex));
            gl.uniform_1_i32(self.present.u("uTex"), 0);
        }
        self.draw();
    }

    pub fn present_transition(&self, old: &Fbo, new: &Fbo, progress: f32, kind: i32, fb: Option<&Fbo>, w: i32, h: i32) {
        let p = &self.transition;
        unsafe {
            let gl = &self.gl;
            gl.bind_framebuffer(glow::FRAMEBUFFER, fb.map(|f| f.fb));
            gl.viewport(0, 0, w, h);
            gl.disable(glow::BLEND);
            gl.use_program(Some(p.id));
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(old.tex));
            gl.active_texture(glow::TEXTURE1);
            gl.bind_texture(glow::TEXTURE_2D, Some(new.tex));
            gl.uniform_1_i32(p.u("uOld"), 0);
            gl.uniform_1_i32(p.u("uNew"), 1);
            gl.uniform_1_f32(p.u("uProgress"), progress);
            gl.uniform_1_i32(p.u("uKind"), kind);
            gl.active_texture(glow::TEXTURE0);
        }
        self.draw();
    }

    /// Destello transparente (alfa premultiplicado) sobre la pantalla o un framebuffer.
    pub fn draw_destello(&self, progress: f32, kind: i32, fb: Option<&Fbo>, w: i32, h: i32, clear: bool) {
        let p = &self.destello;
        unsafe {
            let gl = &self.gl;
            gl.bind_framebuffer(glow::FRAMEBUFFER, fb.map(|f| f.fb));
            gl.viewport(0, 0, w, h);
            if clear {
                gl.clear_color(0.0, 0.0, 0.0, 0.0);
                gl.clear(glow::COLOR_BUFFER_BIT);
                gl.disable(glow::BLEND);
            } else {
                gl.enable(glow::BLEND);
                gl.blend_func(glow::ONE, glow::ONE_MINUS_SRC_ALPHA);
            }
            gl.use_program(Some(p.id));
            gl.uniform_1_f32(p.u("uProgress"), progress);
            gl.uniform_1_i32(p.u("uKind"), kind);
            gl.uniform_2_f32(p.u("uSize"), w as f32, h as f32);
        }
        self.draw();
        unsafe { self.gl.disable(glow::BLEND) };
    }

    pub fn read_rgba(&self, f: &Fbo) -> Vec<u8> {
        let mut px = vec![0u8; (f.w * f.h * 4) as usize];
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(f.fb));
            self.gl.read_pixels(0, 0, f.w, f.h, glow::RGBA, glow::UNSIGNED_BYTE,
                glow::PixelPackData::Slice(Some(&mut px)));
        }
        // OpenGL lee de abajo arriba; el PNG va de arriba abajo
        let row = (f.w * 4) as usize;
        let mut out = Vec::with_capacity(px.len());
        for y in (0..f.h as usize).rev() {
            out.extend_from_slice(&px[y * row..(y + 1) * row]);
        }
        out
    }
}

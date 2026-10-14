//! fondo-vivo: fondos de pantalla animados para los estilos de COSMIC.
//!
//! Sin argumentos se queda en marcha y anima el fondo del estilo puesto
//! (lo lee de ~/.local/share/estilos/.actual). Otros usos:
//!
//!   fondo-vivo captura ESTILO SALIDA.png [ANCHO ALTO SEGUNDO]
//!   fondo-vivo captura-transicion DE A PROGRESO SALIDA.png [ANCHO ALTO]
//!   fondo-vivo captura-destello ESTILO PROGRESO SALIDA.png [ANCHO ALTO]
//!   fondo-vivo comprobar        compila todas las escenas y dice si hay errores
//!   fondo-vivo estilos          lista los estilos con fondo animado

mod daemon;
mod gfx;
mod render;

use gfx::Gfx;
use render::{kind_of, Fbo, Renderer, ESCENAS};
use std::fs::File;

fn write_png(path: &str, w: i32, h: i32, rgba: &[u8]) -> Result<(), String> {
    let file = File::create(path).map_err(|e| format!("{path}: {e}"))?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut wr = enc.write_header().map_err(|e| e.to_string())?;
    wr.write_image_data(rgba).map_err(|e| e.to_string())
}

fn offscreen() -> Result<Renderer, String> {
    let gfx = Box::leak(Box::new(Gfx::new(None)?));
    if !gfx.make_current(None) {
        return Err("no se pudo activar la tarjeta gráfica sin pantalla".into());
    }
    Renderer::new(gfx.glow())
}

fn num<T: std::str::FromStr>(args: &[String], i: usize, default: T) -> T {
    args.get(i).and_then(|s| s.parse().ok()).unwrap_or(default)
}

fn need_scene(r: &mut Renderer, style: &str) -> Result<(), String> {
    if r.has_scene(style) { Ok(()) } else { Err(format!("el estilo «{style}» no tiene fondo animado")) }
}

fn scene_fbo(r: &mut Renderer, style: &str, w: i32, h: i32, t: f32) -> Result<Fbo, String> {
    need_scene(r, style)?;
    let mut f = None;
    r.ensure_fbo(&mut f, w, h);
    let f = f.unwrap();
    r.draw_scene(style, &f, t);
    Ok(f)
}

fn run(args: &[String]) -> Result<(), String> {
    match args.first().map(String::as_str) {
        None => {
            // una sola copia en marcha
            let dir = std::env::var_os("XDG_RUNTIME_DIR").unwrap_or_else(|| "/tmp".into());
            let lock = File::create(std::path::Path::new(&dir).join("fondo-vivo.lock")).map_err(|e| e.to_string())?;
            if lock.try_lock().is_err() {
                println!("fondo-vivo ya está en marcha");
                return Ok(());
            }
            let r = daemon::run();
            drop(lock);
            r
        }
        Some("captura") if args.len() >= 3 => {
            let (w, h, t) = (num(args, 3, 1920), num(args, 4, 1080), num(args, 5, 8.0f32));
            let mut r = offscreen()?;
            let f = scene_fbo(&mut r, &args[1], w, h, t)?;
            write_png(&args[2], w, h, &r.read_rgba(&f))
        }
        Some("captura-transicion") if args.len() >= 5 => {
            let p: f32 = num(args, 3, 0.5);
            let (w, h) = (num(args, 5, 1280), num(args, 6, 720));
            let mut r = offscreen()?;
            let a = scene_fbo(&mut r, &args[1], w, h, 8.0)?;
            let b = scene_fbo(&mut r, &args[2], w, h, 8.0)?;
            let mut out = None;
            r.ensure_fbo(&mut out, w, h);
            let out = out.unwrap();
            r.present_transition(&a, &b, p, kind_of(&args[2]), Some(&out), w, h);
            write_png(&args[4], w, h, &r.read_rgba(&out))
        }
        Some("captura-destello") if args.len() >= 4 => {
            let p: f32 = num(args, 2, 0.5);
            let (w, h) = (num(args, 4, 1280), num(args, 5, 720));
            let mut r = offscreen()?;
            let bg = scene_fbo(&mut r, &args[1], w, h, 8.0)?;
            let mut out = None;
            r.ensure_fbo(&mut out, w, h);
            let out = out.unwrap();
            r.present(&bg, Some(&out), w, h);
            r.draw_destello(p, kind_of(&args[1]), Some(&out), w, h, false);
            write_png(&args[3], w, h, &r.read_rgba(&out))
        }
        Some("comprobar") => {
            let mut r = offscreen()?;
            let mut ok = true;
            for (name, _) in ESCENAS {
                let good = r.has_scene(name);
                println!("{name}: {}", if good { "bien" } else { "ERROR" });
                ok &= good;
            }
            if ok { Ok(()) } else { Err("alguna escena tiene errores".into()) }
        }
        Some("estilos") => {
            for (name, _) in ESCENAS {
                println!("{name}");
            }
            Ok(())
        }
        Some("--version" | "-V") => {
            println!("fondo-vivo {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        _ => Err("uso: fondo-vivo [captura | captura-transicion | captura-destello | comprobar | estilos]".into()),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&args) {
        eprintln!("fondo-vivo: {e}");
        std::process::exit(1);
    }
}

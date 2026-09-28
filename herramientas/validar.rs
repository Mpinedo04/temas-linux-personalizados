//! Validador de temas COSMIC.
//!
//! Carga cada `.ron` con el mismo código que usa Configuración → Apariencia →
//! Importar (`ron::de::from_str::<ThemeBuilder>`), construye el tema final y
//! muestra, para cada zona, el color del texto y el del fondo ya calculados.
//! Si un archivo tiene un error, sale `PARSE_ERROR` con el motivo.
//!
//! Uso (hace falta Rust):
//!   git clone --depth 1 https://github.com/pop-os/libcosmic
//!   cd libcosmic && git submodule update --init --depth 1 iced
//!   cp ../herramientas/validar.rs cosmic-theme/examples/validate.rs
//!   cargo run -q -p cosmic-theme --example validate -- "../Temas de color/"*.ron

use cosmic_theme::composite::over;
use cosmic_theme::{Component, Container, ThemeBuilder};
use palette::{Srgba, WithAlpha};

fn hex(c: Srgba) -> String {
    let c: Srgba<u8> = c.into_format();
    format!("#{:02X}{:02X}{:02X}{:02X}", c.red, c.green, c.blue, c.alpha)
}

fn pair(label: &str, fg: Srgba, bg: Srgba, under: Srgba) {
    // flatten translucent colours onto what's underneath them
    let bg_flat = over(bg, under);
    let fg_flat = over(fg, bg_flat);
    println!("{label}\t{}\t{}", hex(fg_flat), hex(bg_flat));
}

fn comp(label: &str, c: &Component, under: Srgba) {
    pair(label, c.on, c.base, under);
}

fn cont(label: &str, c: &Container, under: Srgba) {
    pair(&format!("{label}.on"), c.on, c.base, under);
    comp(&format!("{label}.component"), &c.component, over(c.base, under));
}

fn main() {
    for path in std::env::args().skip(1) {
        let s = std::fs::read_to_string(&path).unwrap();
        let b: ThemeBuilder = match ron::de::from_str(&s) {
            Ok(b) => b,
            Err(e) => {
                println!("FILE\t{path}\tPARSE_ERROR {e}");
                continue;
            }
        };
        // re-serialise to prove the builder round-trips cleanly
        let _ = ron::ser::to_string(&b).unwrap();
        let t = b.build();
        println!("FILE\t{path}\t{}\tdark={}", t.name, t.is_dark);
        let opaque = t.background(false).base.with_alpha(1.0);
        cont("background", t.background(false), opaque);
        cont("primary", t.primary(false), opaque);
        cont("secondary", t.secondary(false), opaque);
        let bg = over(t.primary(false).base, opaque);
        comp("button", &t.button, bg);
        comp("accent_button", &t.accent_button, bg);
        comp("success_button", &t.success_button, bg);
        comp("destructive_button", &t.destructive_button, bg);
        comp("warning_button", &t.warning_button, bg);
        pair("link_text", t.link_button.on, bg, bg);
        pair("accent_on_primary", t.accent.base, bg, bg);
        pair("input_text(neutral_9)", t.palette.neutral_9, bg, bg);
        pair("warning_on_primary", t.warning.base, bg, bg);
        pair("destructive_on_primary", t.destructive.base, bg, bg);
        pair("success_on_primary", t.success.base, bg, bg);
        println!("INFO\taccent={} bg={} primary={} secondary={} divider={}", hex(t.accent.base), hex(t.background(false).base), hex(t.primary(false).base), hex(t.secondary(false).base), hex(t.background(false).divider));
    }
}

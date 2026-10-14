//! EGL: abre la tarjeta gráfica (sobre Wayland o sin pantalla, para capturas).

use khronos_egl as egl;
use std::ffi::c_void;

pub type Egl = egl::DynamicInstance<egl::EGL1_5>;

const PLATFORM_WAYLAND_KHR: egl::Enum = 0x31D8;
const PLATFORM_SURFACELESS_MESA: egl::Enum = 0x31DD;

pub struct Gfx {
    pub egl: Egl,
    pub display: egl::Display,
    pub config: egl::Config,
    pub context: egl::Context,
}

impl Gfx {
    /// `wl_display` es el puntero de la conexión Wayland; `None` para trabajar sin pantalla.
    pub fn new(wl_display: Option<*mut c_void>) -> Result<Self, String> {
        let egl = unsafe { Egl::load_required_from_filename("libEGL.so.1") }
            .map_err(|e| format!("no se pudo cargar libEGL.so.1: {e}"))?;
        let display = unsafe {
            match wl_display {
                Some(ptr) => egl.get_platform_display(PLATFORM_WAYLAND_KHR, ptr, &[egl::ATTRIB_NONE]),
                None => egl.get_platform_display(
                    PLATFORM_SURFACELESS_MESA,
                    std::ptr::null_mut(),
                    &[egl::ATTRIB_NONE],
                ),
            }
        }
        .map_err(|e| format!("EGL sin pantalla: {e}"))?;
        egl.initialize(display).map_err(|e| format!("EGL initialize: {e}"))?;
        egl.bind_api(egl::OPENGL_ES_API).map_err(|e| format!("EGL bind_api: {e}"))?;

        let surface_type = if wl_display.is_some() { egl::WINDOW_BIT } else { egl::PBUFFER_BIT };
        let attrs = [
            egl::RED_SIZE, 8,
            egl::GREEN_SIZE, 8,
            egl::BLUE_SIZE, 8,
            egl::ALPHA_SIZE, 8,
            egl::SURFACE_TYPE, surface_type,
            egl::RENDERABLE_TYPE, egl::OPENGL_ES3_BIT,
            egl::NONE,
        ];
        let config = egl
            .choose_first_config(display, &attrs)
            .map_err(|e| format!("EGL choose_config: {e}"))?
            .ok_or("la tarjeta gráfica no ofrece OpenGL ES 3")?;
        let ctx_attrs = [egl::CONTEXT_MAJOR_VERSION, 3, egl::CONTEXT_MINOR_VERSION, 0, egl::NONE];
        let context = egl
            .create_context(display, config, None, &ctx_attrs)
            .map_err(|e| format!("EGL create_context: {e}"))?;
        Ok(Self { egl, display, config, context })
    }

    pub fn make_current(&self, surface: Option<egl::Surface>) -> bool {
        self.egl.make_current(self.display, surface, surface, Some(self.context)).is_ok()
    }

    pub fn glow(&self) -> glow::Context {
        unsafe {
            glow::Context::from_loader_function(|s| {
                self.egl.get_proc_address(s).map_or(std::ptr::null(), |f| f as *const c_void)
            })
        }
    }

    pub fn create_window_surface(&self, native: *const c_void) -> Option<egl::Surface> {
        unsafe {
            self.egl
                .create_window_surface(self.display, self.config, native as egl::NativeWindowType, None)
                .map_err(|e| eprintln!("fondo-vivo: create_window_surface: {e}"))
                .ok()
        }
    }

    pub fn destroy_surface(&self, surface: egl::Surface) {
        let _ = self.egl.make_current(self.display, None, None, Some(self.context));
        let _ = self.egl.destroy_surface(self.display, surface);
    }
}

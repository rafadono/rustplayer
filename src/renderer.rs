use glow::HasContext;
use libloading::Library;
use libmpv2::{
    render::{OpenGLInitParams, RenderContext, RenderParam, RenderParamApiType},
    Mpv,
};
use std::ffi::{c_void, CString};
use std::sync::{Arc, Mutex};

struct GlProcResolver {
    #[cfg(target_os = "windows")]
    gl_lib: Library,
    #[cfg(target_os = "windows")]
    wgl_get_proc_address: unsafe extern "system" fn(*const i8) -> *const c_void,

    #[cfg(target_os = "linux")]
    gl_lib: Library,
    #[cfg(target_os = "linux")]
    glx_get_proc_address: unsafe extern "C" fn(*const u8) -> *const c_void,
}

impl GlProcResolver {
    fn new() -> Result<Self, String> {
        #[cfg(target_os = "windows")]
        {
            let gl_lib = unsafe { Library::new("opengl32.dll") }
                .map_err(|e| format!("No se pudo abrir opengl32.dll: {e}"))?;
            let wgl_get_proc_address = unsafe {
                *gl_lib
                    .get::<unsafe extern "system" fn(*const i8) -> *const c_void>(
                        b"wglGetProcAddress\0",
                    )
                    .map_err(|e| format!("No se pudo obtener wglGetProcAddress: {e}"))?
            };
            return Ok(Self {
                gl_lib,
                wgl_get_proc_address,
            });
        }

        #[cfg(target_os = "linux")]
        {
            let gl_lib = unsafe { Library::new("libGL.so.1") }
                .or_else(|_| unsafe { Library::new("libGL.so") })
                .map_err(|e| format!("No se pudo abrir libGL: {e}"))?;

            let glx_get_proc_address = unsafe {
                if let Ok(sym) = gl_lib.get::<unsafe extern "C" fn(*const u8) -> *const c_void>(
                    b"glXGetProcAddressARB\0",
                ) {
                    *sym
                } else {
                    *gl_lib
                        .get::<unsafe extern "C" fn(*const u8) -> *const c_void>(
                            b"glXGetProcAddress\0",
                        )
                        .map_err(|e| format!("No se pudo obtener glXGetProcAddress: {e}"))?
                }
            };

            return Ok(Self {
                gl_lib,
                glx_get_proc_address,
            });
        }

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            Err("Plataforma no soportada por el renderer OpenGL de MPV".to_string())
        }
    }
}

fn get_proc_address(ctx: &Arc<GlProcResolver>, name: &str) -> *mut c_void {
    let cname = match CString::new(name) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    #[cfg(target_os = "windows")]
    {
        let ptr = unsafe { (ctx.wgl_get_proc_address)(cname.as_ptr()) } as *mut c_void;
        let invalid = ptr.is_null()
            || ptr as usize == 1
            || ptr as usize == 2
            || ptr as usize == 3
            || ptr as isize == -1;
        if !invalid {
            return ptr;
        }
        return unsafe {
            match ctx.gl_lib.get::<*const c_void>(cname.as_bytes_with_nul()) {
                Ok(sym) => *sym as *mut c_void,
                Err(_) => std::ptr::null_mut(),
            }
        };
    }

    #[cfg(target_os = "linux")]
    {
        let ptr = unsafe { (ctx.glx_get_proc_address)(cname.as_ptr() as *const u8) } as *mut c_void;
        if !ptr.is_null() {
            return ptr;
        }
        return unsafe {
            match ctx.gl_lib.get::<*const c_void>(cname.as_bytes_with_nul()) {
                Ok(sym) => *sym as *mut c_void,
                Err(_) => std::ptr::null_mut(),
            }
        };
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        let _ = ctx;
        std::ptr::null_mut()
    }
}

pub struct MpvRenderer {
    render_context: Arc<Mutex<UnsafeRenderContext>>,
    offscreen: Arc<Mutex<Option<OffscreenBuffer>>>,
}

struct OffscreenBuffer {
    fbo: glow::Framebuffer,
    tex: glow::Texture,
    width: i32,
    height: i32,
}

struct UnsafeRenderContext(RenderContext);
unsafe impl Send for UnsafeRenderContext {}
unsafe impl Sync for UnsafeRenderContext {}

fn get_or_create_offscreen(
    gl: &glow::Context,
    offscreen_mutex: &Mutex<Option<OffscreenBuffer>>,
    width: i32,
    height: i32,
) -> Result<glow::Framebuffer, String> {
    let mut offscreen = offscreen_mutex.lock().unwrap();
    if let Some(ref buf) = *offscreen {
        if buf.width == width && buf.height == height {
            return Ok(buf.fbo);
        }
        // Destroy old buffer
        unsafe {
            gl.delete_framebuffer(buf.fbo);
            gl.delete_texture(buf.tex);
        }
    }

    unsafe {
        let tex = gl
            .create_texture()
            .map_err(|e| format!("Error al crear textura: {e}"))?;
        gl.bind_texture(glow::TEXTURE_2D, Some(tex));
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            width,
            height,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            None,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MIN_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_MAG_FILTER,
            glow::LINEAR as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_S,
            glow::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );

        let fbo = gl
            .create_framebuffer()
            .map_err(|e| format!("Error al crear FBO: {e}"))?;
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
        gl.framebuffer_texture_2d(
            glow::FRAMEBUFFER,
            glow::COLOR_ATTACHMENT0,
            glow::TEXTURE_2D,
            Some(tex),
            0,
        );

        let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);
        if status != glow::FRAMEBUFFER_COMPLETE {
            gl.delete_framebuffer(fbo);
            gl.delete_texture(tex);
            return Err(format!("Framebuffer incompleto: {status}"));
        }

        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        gl.bind_texture(glow::TEXTURE_2D, None);

        let buf = OffscreenBuffer {
            fbo,
            tex,
            width,
            height,
        };
        *offscreen = Some(buf);
        Ok(fbo)
    }
}

impl MpvRenderer {
    pub fn new(_gl: &glow::Context, mpv: Arc<Mpv>) -> Result<Self, String> {
        let resolver = Arc::new(GlProcResolver::new()?);
        let params = vec![
            RenderParam::ApiType(RenderParamApiType::OpenGl),
            RenderParam::InitParams(OpenGLInitParams {
                get_proc_address,
                ctx: resolver,
            }),
        ];

        let render_context = unsafe { RenderContext::new(&mut *mpv.ctx.as_ptr(), params) }
            .map_err(|e| format!("No se pudo crear RenderContext: {e}"))?;

        Ok(Self {
            render_context: Arc::new(Mutex::new(UnsafeRenderContext(render_context))),
            offscreen: Arc::new(Mutex::new(None)),
        })
    }

    pub fn destroy_gl_resources(&self, gl: &glow::Context) {
        let mut offscreen = self.offscreen.lock().unwrap();
        if let Some(buf) = offscreen.take() {
            unsafe {
                gl.delete_framebuffer(buf.fbo);
                gl.delete_texture(buf.tex);
            }
        }
    }

    pub fn paint_callback(
        &self,
        rect: egui::Rect,
        screen_rect: egui::Rect,
        _pixels_per_point: f32,
    ) -> egui::PaintCallback {
        let render_context = Arc::clone(&self.render_context);
        let offscreen = Arc::clone(&self.offscreen);
        egui::PaintCallback {
            rect,
            callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                let ppp = info.pixels_per_point;
                let x = (rect.min.x * ppp).round() as i32;
                let y = ((screen_rect.height() - rect.max.y) * ppp).round() as i32;
                let width = (rect.width() * ppp).round().max(1.0) as i32;
                let height = (rect.height() * ppp).round().max(1.0) as i32;

                if width <= 0 || height <= 0 {
                    return;
                }

                let gl = painter.gl();
                let target_fbo = unsafe { gl.get_parameter_i32(glow::FRAMEBUFFER_BINDING) };

                // Obtain or create the offscreen FBO
                let offscreen_fbo = match get_or_create_offscreen(gl, &offscreen, width, height) {
                    Ok(fbo) => fbo,
                    Err(e) => {
                        log::error!("FBO offscreen error: {}", e);
                        return;
                    }
                };

                // Render the video on the FBO offscreen
                if let Ok(ctx) = render_context.lock() {
                    let _ = ctx.0.render::<Arc<GlProcResolver>>(
                        offscreen_fbo.0.get() as i32,
                        width,
                        height,
                        true,
                    );
                    ctx.0.report_swap();
                }

                // Copy from offscreen FBO to destination FBO
                unsafe {
                    gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(offscreen_fbo));

                    let target_fbo_opt = if target_fbo == 0 {
                        None
                    } else {
                        std::num::NonZeroU32::new(target_fbo as u32).map(glow::NativeFramebuffer)
                    };
                    gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, target_fbo_opt);

                    gl.blit_framebuffer(
                        0,
                        0,
                        width,
                        height,
                        x,
                        y,
                        x + width,
                        y + height,
                        glow::COLOR_BUFFER_BIT,
                        glow::LINEAR,
                    );

                    // Restore the original framebuffer
                    gl.bind_framebuffer(glow::FRAMEBUFFER, target_fbo_opt);
                }
            })),
        }
    }
}

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use wgpu;
use winit::{dpi::{PhysicalSize, LogicalSize}, event_loop::EventLoop, window::Window};

pub struct GfxWindow {
    pub window: Window,
    pub event_loop: EventLoop<()>,
}

impl GfxWindow {
    pub fn new(width: i32, height: i32) -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();

        window.set_inner_size(LogicalSize::new(width, height));
        Self { window, event_loop }
    }
}

pub struct GfxInstance {
    #[cfg(target_arch="wasm32")]
    on_update: js_sys::Function,

    #[cfg(target_arch="wasm32")]
    on_render: js_sys::Function,

    #[cfg(target_arch="wasm32")]
    on_resize: js_sys::Function,

    clear_color: wgpu::Color,

    /// The main wgpu instance.
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl GfxInstance {
   
    pub fn new(
        #[cfg(target_arch="wasm32")]
        on_update: js_sys::Function,
        #[cfg(target_arch="wasm32")]
        on_render: js_sys::Function,
        #[cfg(target_arch="wasm32")] 
        on_resize: js_sys::Function,
        window: *mut GfxWindow
    ) -> Self {

        let w = unsafe { &*window };

        #[cfg(target_arch = "wasm32")]
        {
            // Winit prevents sizing with CSS, so we have to set
            // the size manually when on web.
            use winit::dpi::LogicalSize;

            web_sys::console::log_1(&"Hello using web-sys".into());
            use winit::platform::web::WindowExtWebSys;

            let mut html_body: Option<web_sys::HtmlElement> = None;

            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    html_body = doc.body();

                    let width  = html_body.as_ref().unwrap().client_width();
                    let height = html_body.as_ref().unwrap().client_height();
                    
                    w.window.set_inner_size(LogicalSize::new(width, height));
                    
                    let dst = doc.get_element_by_id("wasm-example")?;
                    let canvas = web_sys::Element::from(w.window.canvas());
                    dst.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Couldn't append canvas to document body.");
        }

        let size = w.window.inner_size();
        
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("window: {:?}", &w.window).into());

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Create instance...").into());

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Create surface...").into());

        let surface = unsafe { instance.create_surface(&w.window) };

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Create adapter...").into());

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        );

        let adapter = pollster::block_on(adapter).unwrap();

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Create device and queue...").into());

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        )).unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Configure surface...").into());
        surface.configure(&device, &config);

        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("Initialized...").into());

        Self {
            #[cfg(target_arch="wasm32")]
            on_update,
            
            #[cfg(target_arch="wasm32")]
            on_render,
            
            #[cfg(target_arch="wasm32")]
            on_resize,

            instance,
            surface,
            device,
            queue,
            config,
            size,

            clear_color: wgpu::Color { r: 0.01, g: 0.01, b: 0.01, a: 1f64 }
        }
    }

    pub fn update(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                self.on_update.apply(&JsValue::null(), &js_sys::Array::new()).unwrap();
            } else {
                todo!("Implement Update for C/C++");
            }
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();         

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                self.on_render.apply(&JsValue::null(), &js_sys::Array::new()).unwrap();
            } else {
                // todo!("Implement Render for C/C++");
            }
        }

        Ok(())
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, logical_size: LogicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size           = new_size;
            self.config.width   = new_size.width;
            self.config.height  = new_size.height;

            self.surface.configure(&self.device, &self.config);
        }

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let args = js_sys::Array::new_with_length(4);
                args.set(0, new_size.width.into());
                args.set(1, new_size.height.into());

                args.set(2, logical_size.width.into());
                args.set(3, logical_size.height.into());

                self.on_resize.apply(&JsValue::null(), &args).unwrap();
            } else {
                todo!("Implement on resize for C/C++");
            }
        }
    }
    
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn gfx_create_window() -> *mut GfxWindow {
    let ptr = std::boxed::Box::new(GfxWindow::new(800, 600));
    std::boxed::Box::into_raw(ptr)
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn gfx_create_instance(
    on_update: js_sys::Function, 
    on_render: js_sys::Function, 
    on_resize: js_sys::Function,
    window_ptr: *mut GfxWindow,
) -> *mut GfxInstance {
    let ptr = std::boxed::Box::new(GfxInstance::new(on_update, on_render, on_resize, window_ptr));
    std::boxed::Box::into_raw(ptr)
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn gfx_detroy_instance(instance: *mut GfxInstance) {
    unsafe { std::boxed::Box::from_raw(instance); }
    web_sys::console::log_1(&format!("GraphX instance was successfully destoryed !").into());
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn gfx_update(instance: *mut GfxInstance) {
    let instance = unsafe { &mut *instance };
    instance.update();
}


/// Set the clear color.
/// 
/// # Arguments
/// 
/// * `instance` - The pointer to the GraphX instance
/// * `r` - Red value (from `0` to `1`)
/// * `g` - Green value (from `0` to `1`)
/// * `b` - Blue value (from `0` to `1`)
/// 
#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn gfx_set_clear_color(instance: *mut GfxInstance, r: f64, g: f64, b: f64, a: f64) {
    let instance = unsafe { &mut *instance };
    instance.clear_color = wgpu::Color { r, g, b, a };
}


#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn gfx_test_struct(s: &js_sys::Object) {
    let val = js_sys::Reflect::get(s, &"name".into()).unwrap();
    web_sys::console::log_1(&format!("val: {}", val.as_string().unwrap()).into());
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn gfx_create_shader(source: &str) {
    web_sys::console::log_1(&format!("source: {}", source).into());
}
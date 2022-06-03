use winit::dpi::{PhysicalSize, LogicalSize};
use super::window::IlsWindow;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub struct IlsInstance {
    /// Called once per frame.
    /// 
    /// You must do all your game
    /// logic in this callback.
    #[cfg(target_arch="wasm32")]
    on_update: js_sys::Function,

    /// Called once per frame after
    /// `on_update`.
    /// 
    /// You must do all your render
    /// operations in this callback.
    #[cfg(target_arch="wasm32")]
    on_render: js_sys::Function,

    /// Called when the canvas is
    /// resized.
    #[cfg(target_arch="wasm32")]
    on_resize: js_sys::Function,

    /// The canvas clear color.
    clear_color: wgpu::Color,
    
    /// The current surface size.
    size: winit::dpi::PhysicalSize<u32>,

    #[cfg(target_arch="wasm32")]
    scale_factor: f64,

    instance:   wgpu::Instance,
    surface:    wgpu::Surface,
    device:     wgpu::Device,
    queue:      wgpu::Queue,
    config:     wgpu::SurfaceConfiguration,
}

impl IlsInstance {
    /// Create a new Illusion instance.
    pub fn new(
        #[cfg(target_arch="wasm32")]
        params: &js_sys::Object,
        window: &IlsWindow
    ) -> Self {
        
        let size = window.get_inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window.get_window()) };

        #[cfg(target_arch="wasm32")]
        let scale_factor: f64 = window.get_window().scale_factor();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        );

        let adapter = pollster::block_on(adapter).unwrap();

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
            usage:          wgpu::TextureUsages::RENDER_ATTACHMENT,
            format:         surface.get_preferred_format(&adapter).unwrap(),
            width:          size.width,
            height:         size.height,
            present_mode:   wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        Self {
            #[cfg(target_arch="wasm32")]
            on_update: js_sys::Function::from(js_sys::Reflect::get(&params, &"on_update".into()).unwrap()),
            #[cfg(target_arch="wasm32")]
            on_render: js_sys::Function::from(js_sys::Reflect::get(&params, &"on_render".into()).unwrap()),
            #[cfg(target_arch="wasm32")]
            on_resize: js_sys::Function::from(js_sys::Reflect::get(&params, &"on_resize".into()).unwrap()),
            #[cfg(target_arch="wasm32")]
            scale_factor,

            instance,
            surface,
            device,
            queue,
            config,
            size,

            clear_color: wgpu::Color { r: 0.01, g: 0.01, b: 0.01, a: 1f64 },
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

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
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

                let logical_size: LogicalSize<u32> = new_size.to_logical(self.scale_factor);

                args.set(2, logical_size.width.into());
                args.set(3, logical_size.height.into());

                self.on_resize.apply(&JsValue::null(), &args).unwrap();
            } else {
                todo!("Implement on resize for C/C++");
            }
        }
    }

    pub fn get_size(&self) -> &PhysicalSize<u32> {
        &self.size
    }

    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.clear_color = wgpu::Color { r, g, b, a };
    }

}
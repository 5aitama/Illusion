use winit::{window::{Window, WindowBuilder}, event_loop::EventLoop, dpi::{LogicalSize, PhysicalSize}};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowBuilderExtWebSys;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

pub struct IlsWindow {
    /// The window itself
    pub window: Window,
    
    /// The window event loop
    pub event_loop: EventLoop<()>,
}

impl IlsWindow {
    /// Create new window.
    /// 
    /// # Arguments
    /// 
    /// * `width` - The window width (in pixels)
    /// * `height` - The window height (in pixels)
    pub fn new(width: i32, height: i32) -> Self {
        let event_loop = EventLoop::new();
        
        cfg_if::cfg_if! {
            // For wasm we need to create the window
            // from the canvas...
            if #[cfg(target_arch = "wasm32")] {
                let mut builder = WindowBuilder::new();
                let canvas_element_id = "main-canvas";

                builder = web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let canvas = doc.get_element_by_id(canvas_element_id)?.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
                    Some(builder.with_canvas(Some(canvas.into())))
                }).expect(format!("Couldn't retrieve canvas width id {} from document body.", &canvas_element_id).as_str());
            } else {
                let builder = WindowBuilder::new();
            }
        }

        let window = builder.build(&event_loop).unwrap();
        window.set_inner_size(LogicalSize::new(width, height));
        
        Self { window, event_loop }
    }

    /// Get the window's inner size.
    /// 
    /// # Returns
    /// 
    /// The window's inner size.
    pub fn get_inner_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

}
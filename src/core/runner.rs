use winit::{
    event::*,
    event_loop::ControlFlow
};

use super::{instance::IlsInstance, window::IlsWindow};

pub fn run(mut instance: IlsInstance, win: IlsWindow) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = win.event_loop;
    let window = win.window;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            
            WindowEvent::Resized(physical_size) => {
                instance.resize(*physical_size);
            },

            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                instance.resize(**new_inner_size);
            },

            WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },

        Event::RedrawRequested(window_id) if window_id == window.id() => {
            instance.update();
            match instance.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => instance.resize(*instance.get_size()),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        },

        Event::MainEventsCleared => {
            #[cfg(target_arch = "wasm32")] {
                let body = web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.body());
                
                let width  = body.as_ref().unwrap().client_width();
                let height = body.as_ref().unwrap().client_height();
                
                let logical_size = window.inner_size().to_logical::<i32>(window.scale_factor());

                if width != logical_size.width || height != logical_size.height {
                    window.set_inner_size(winit::dpi::LogicalSize::new(width, height));
                    web_sys::console::log_1(&format!("Resize width: {}, height: {}", &width, &height).into());
                }
            }

            window.request_redraw();
        }
        _ => {}
    });
}
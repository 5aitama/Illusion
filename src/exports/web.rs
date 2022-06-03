#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch="wasm32")]
use crate::core::{instance::IlsInstance, window::IlsWindow};

#[cfg(target_arch="wasm32")]
use crate::core::runner::run;

// #[cfg(target_arch="wasm32")]
// #[wasm_bindgen]
// /// Start the main loop of an Illusion instance.
// pub fn ils_instance_run(instance: *mut GfxInstance, window: *mut GfxWindow) {

//     let instance = *unsafe { std::boxed::Box::from_raw(instance) };
//     let window   = *unsafe { std::boxed::Box::from_raw(window)   };

//     // run(instance, window);
// }

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn ils_create_window(width: i32, height: i32) -> *mut IlsWindow {
    use crate::core::window::IlsWindow;

    let ptr = std::boxed::Box::new(IlsWindow::new(width, height));
    std::boxed::Box::into_raw(ptr)
}


#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn ils_create_instance(window: *const IlsWindow, params: &js_sys::Object) -> *mut IlsInstance {
    let window = unsafe { &*window };
    let ptr = std::boxed::Box::new(IlsInstance::new(params, &window));

    std::boxed::Box::into_raw(ptr)
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn ils_run_instance(window: *mut IlsWindow, instance: *mut IlsInstance) {
    let window = *unsafe { std::boxed::Box::from_raw(window) };
    let instance = *unsafe { std::boxed::Box::from_raw(instance) };

    run(instance, window);
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
pub fn ils_set_clear_color(instance: *mut IlsInstance, r: f64, g: f64, b: f64, a: f64) {
    let mut instance = unsafe { &mut *instance };
    instance.set_clear_color(r, g, b, a);
}
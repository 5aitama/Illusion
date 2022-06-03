pub mod instance;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch="wasm32")]
use instance::gfx::{GfxInstance, GfxWindow};

#[cfg(target_arch="wasm32")]
use instance::core::run;

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn gfx_start(instance: *mut GfxInstance, window: *mut GfxWindow) {

    let instance = *unsafe { std::boxed::Box::from_raw(instance) };
    let window   = *unsafe { std::boxed::Box::from_raw(window)   };

    run(instance, window);
}
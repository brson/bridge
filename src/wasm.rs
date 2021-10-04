use log::info;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug);

    info!("bridge wasm initialized");
}

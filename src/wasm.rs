use log::info;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug);

    info!("bridge wasm initialized");
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    name: String,
}

pub type Json = String;

#[wasm_bindgen]
pub fn new_game() -> Json {
    serialize(new_game_())
}

fn new_game_() -> Game {
    Game {
        name: "hi".to_string()
    }
}

fn serialize<T>(value: T) -> Json
where T: Serialize
{
    serde_json::to_string(&value).expect("json")
}

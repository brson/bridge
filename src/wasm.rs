use serde::{Serialize, Deserialize};
use serde_json;

mod exports {
    use wasm_bindgen::prelude::*;
    use super::{api, serialize};

    pub type Json = String;

    #[wasm_bindgen]
    pub fn init() {
        api::init()
    }

    #[wasm_bindgen]
    pub fn new_game() -> Json {
        serialize(api::new_game())
    }
}

mod api {
    use log::info;
    use serde::{Serialize, Deserialize};
    use crate::defs::AuctionState;
    use crate::gen;

    #[derive(Serialize, Deserialize)]
    pub struct Game {
        seed: u32,
        auction: AuctionState,
    }

    pub fn init() {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Debug);

        info!("bridge wasm initialized");
    }

    pub fn new_game() -> Game {
        let seed = 0;
        let auction = gen::random_auction(seed);

        Game {
            seed,
            auction
        }
    }
}

fn serialize<T>(value: T) -> String
where T: Serialize
{
    serde_json::to_string(&value).expect("json")
}

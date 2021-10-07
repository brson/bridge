use serde::{Serialize, Deserialize};
use serde_json;

mod exports {
    use wasm_bindgen::prelude::*;
    use super::{api, serialize, deserialize};

    pub type Json = String;

    #[wasm_bindgen]
    pub fn init() {
        api::init()
    }

    #[wasm_bindgen]
    pub fn new_game() -> Json {
        serialize(api::new_game())
    }

    #[wasm_bindgen]
    pub fn card_value_and_suit(card: u8) -> Json {
        serialize(api::card_value_and_suit(card))
    }

    #[wasm_bindgen]
    pub fn next_player(auction_state: Json) -> Json {
        serialize(api::next_player(deserialize(auction_state)))
    }

    #[wasm_bindgen]
    pub fn play_auction_call(game: Json, call: Json) -> Json {
        serialize(api::play_auction_call(
            deserialize(game),
            deserialize(call)
        ))
    }
}

mod api {
    use log::info;
    use serde::{Serialize, Deserialize};
    use crate::defs::*;
    use crate::gen;
    use crate::eval::{self, CallEvaluationResult};

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

    pub fn card_value_and_suit(card: u8) -> (u8, Suit) {
        let card = Card::from(card);
        (card.face_value(), card.suit())
    }

    pub fn next_player(game: Game) -> Seat {
        game.auction.next_player()
    }

    pub fn play_auction_call(game: Game, call: PlayerCall) -> CallEvaluationResult {
        log::info!("playing {:?}", call);
        eval::check_call(game.auction, call)
    }
}

fn serialize<T>(value: T) -> String
where T: Serialize
{
    serde_json::to_string(&value).expect("json")
}

fn deserialize<T>(s: String) -> T
where T: for <'de> Deserialize<'de> + 'static
{
    serde_json::from_str(&s).expect("json")
}

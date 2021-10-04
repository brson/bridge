use crate::defs::*;

pub fn random_auction(seed: u32) -> AuctionState {
    let [north, east, south, west] = deal();

    AuctionState {
        deck: Deck {
            north,
            east,
            south,
            west,
        },
        dealer: Seat::North,
        calls: vec![],
    }
}

fn deal() -> [Hand; 4] {
    [
        deal_one(),
        deal_one(),
        deal_one(),
        deal_one(),
    ]
}

fn deal_one() -> Hand {
    todo!()
}

// http://www.kwbridge.com/basics.htm

use crate::defs::{
    AuctionPlayerView,
    BidSuit,
};

pub struct SimulatedCalls {
}

pub fn simulate_call(view: &AuctionPlayerView) -> SimulatedCalls {
    let player = view.next_player();

    let opening = view.opening();

    if opening {
        let hcps = view.high_card_points();
        let have_12plus_hcps = 12 <= hcps;
        let have_15_to_17_hcps = 15 <= hcps && hcps <= 117;
        let balanced = view.balanced();

        if have_15_to_17_hcps && balanced {
            bid(1, BidSuit::NoTrump)
        } else {
            todo!()
        }
    } else {
        todo!()
    }
}

fn bid(level: u8, suit: BidSuit) -> ! {
    todo!()
}

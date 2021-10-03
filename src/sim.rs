use crate::defs::AuctionPlayerView;

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
        todo!()
    } else {
        todo!()
    }
}


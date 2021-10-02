use crate::defs::AuctionPlayerView;

pub struct SimulatedCalls {
}

pub fn simulate_call(view: &AuctionPlayerView) -> SimulatedCalls {
    let player = view.next_player();

    let opening = view.opening();

    if opening {
        let have_12plus_hcps = view.hcps() >= 12;
        todo!()
    } else {
        todo!()
    }
}


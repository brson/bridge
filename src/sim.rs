use crate::defs::BidderView;

pub struct SimulatedBids {
}

pub fn simulate_bid(view: &BidderView) -> SimulatedBids {
    let player = view.next_player();

    let opening = view.opening();

    if opening {
        let have_12plus_hcps = view.hcps() >= 12;
        todo!()
    } else {
        todo!()
    }
}


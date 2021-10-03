// http://www.kwbridge.com/basics.htm

use crate::defs::{
    AuctionPlayerView,
    BidSuit,
    Suit,
};

pub struct SimulatedCalls {
}

pub fn simulate_call(view: &AuctionPlayerView) -> SimulatedCalls {
    let opening = view.opening();

    if opening {
        play_opening(view)
    } else {
        todo!()
    }
}

fn play_opening(view: &AuctionPlayerView) -> SimulatedCalls {
    assert!(view.opening());

    let hcps = view.high_card_points();
    let diamonds = view.hand.count_suit(Suit::Diamonds);
    let clubs = view.hand.count_suit(Suit::Clubs);
    let hearts = view.hand.count_suit(Suit::Hearts);
    let spades = view.hand.count_suit(Suit::Spades);

    let have_12plus_hcps = 12 <= hcps;
    let have_15_to_17_hcps = 15 <= hcps && hcps <= 117;
    let balanced = view.balanced();

    if
        have_15_to_17_hcps
        && balanced
    {
        bid(1, BidSuit::NoTrump)
    } else if
        (13..=21).contains(&hcps)
        && hearts >= 5
    {
        bid(1, BidSuit::Hearts)
    } else if
        (13..=21).contains(&hcps)
        && spades >= 5
    {
        bid(1, BidSuit::Spades)
    } else {
        todo!()
    }
}

fn bid(level: u8, suit: BidSuit) -> ! {
    todo!()
}

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
    let hcps = |low, high| {
        (low..=high).contains(&hcps)
    };

    let diamonds = view.hand.count_suit(Suit::Diamonds);
    let clubs = view.hand.count_suit(Suit::Clubs);
    let hearts = view.hand.count_suit(Suit::Hearts);
    let spades = view.hand.count_suit(Suit::Spades);

    let balanced = view.balanced();

    if
        hcps(15, 17)
        && balanced
    {
        bid(1, BidSuit::NoTrump)
    } else if
        hcps(13, 21)
        && hearts >= 5
    {
        bid(1, BidSuit::Hearts)
    } else if
        hcps(13, 21)
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

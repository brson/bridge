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

    let five_card_major = hearts >= 5 || clubs >= 4;
    let three_card_minor = diamonds >= 3 || clubs >= 3;

    if
        balanced
        && hcps(15, 17)
    {
        bid(1, BidSuit::NoTrump)
    } else if
        five_card_major
        && hcps(13, 21)
    {
        if hearts >= 4 {
            bid(1, BidSuit::Hearts)
        } else {
            bid(1, BidSuit::Spades)
        }
    } else if
        three_card_minor
        && hcps(13, 21)
    {
        todo!()
    } else {
        todo!()
    }
}

fn bid(level: u8, suit: BidSuit) -> ! {
    todo!()
}

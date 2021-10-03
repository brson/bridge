// http://www.kwbridge.com/basics.htm

use crate::defs::{
    AuctionPlayerView,
    BidSuit,
    Suit,
};

pub struct SimulatedCalls {
}

pub enum BidReason {
    Todo,
    ConvenientMinor,
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

    let clubs = view.hand.count_suit(Suit::Clubs);
    let diamonds = view.hand.count_suit(Suit::Diamonds);
    let hearts = view.hand.count_suit(Suit::Hearts);
    let spades = view.hand.count_suit(Suit::Spades);

    let balanced = view.balanced();
    let five_card_major = hearts >= 5 || spades >= 4;
    let three_card_minor = clubs >= 3 || diamonds >= 3;

    if
        balanced
        && hcps(15, 17)
    {
        bid(1, BidSuit::NoTrump, BidReason::Todo)
    } else if
        five_card_major
        && hcps(13, 21)
    {
        if hearts >= 5 {
            bid(1, BidSuit::Hearts, BidReason::Todo)
        } else {
            bid(1, BidSuit::Spades, BidReason::Todo)
        }
    } else if
        three_card_minor
        && hcps(13, 21)
    {
        if diamonds > clubs {
            bid(1, BidSuit::Diamonds, BidReason::Todo)
        } else {
            // "convenient minor"
            bid(1, BidSuit::Clubs, BidReason::ConvenientMinor)
        }
    } else {
        todo!()
    }
}

fn bid(level: u8, suit: BidSuit, reason: BidReason) -> ! {
    todo!()
}

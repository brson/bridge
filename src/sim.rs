// http://www.kwbridge.com/basics.htm

use crate::defs::{
    AuctionPlayerView,
    BidSuit,
    MAX_HCPS,
    Suit,
};

pub struct SimulatedCalls {
}

pub enum BidReason {
    Todo,
    OpeningConvenientMinor,
    OpeningArtificialVeryStrongHand,
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

    if false {
        unreachable!()
    } else if
        hcps(15, 17)
        && balanced
    {
        bid(1, BidSuit::NoTrump, BidReason::Todo)
    } else if
        hcps(13, 21)
        && five_card_major
    {
        if spades > hearts {
            bid(1, BidSuit::Spades, BidReason::Todo)
        } else {
            bid(1, BidSuit::Hearts, BidReason::Todo)
        }
    } else if
        hcps(13, 21)
        && three_card_minor
    {
        if diamonds > clubs {
            bid(1, BidSuit::Diamonds, BidReason::Todo)
        } else {
            // "convenient minor"
            bid(1, BidSuit::Clubs, BidReason::OpeningConvenientMinor)
        }
    } else if
        hcps(11, 12)
    {
        // marginal cases
        let long_suit = {
            if hearts > 5 && hearts >= spades {
                Some(BidSuit::Hearts)
            } else if spades > 5 {
                Some(BidSuit::Spades)
            } else {
                None
            }
        };
        let distributional_strength = todo!();
        let good_quick_tricks = None; // todo

        if let Some(long_suit) = long_suit {
            bid(1, long_suit, BidReason::Todo)
        } else {
            pass()
        }
    } else if
        hcps(20, 22)
        && balanced
    {
        bid(2, BidSuit::NoTrump, BidReason::Todo)
    } else if
        hcps(23, MAX_HCPS)
    {
        bid(2, BidSuit::Clubs, BidReason::OpeningArtificialVeryStrongHand)
    } else if
        hcps(5, 9)
    {
        // weak hand
        todo!()
    } else if
        hcps(10, 10)
    {
        // don't know any heuristics for 10 hcps
        todo!()
    } else {
        pass()
    }
}

fn bid(level: u8, suit: BidSuit, reason: BidReason) -> SimulatedCalls {
    todo!()
}

fn pass() -> SimulatedCalls {
    todo!()
}

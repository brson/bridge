// http://www.kwbridge.com/basics.htm
// http://www.kwbridge.com/weak2.htm

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
    OpeningBalanced15To17(OpeningBalanced15To17),
    OpeningFiveCardMajor(OpeningFiveCardMajor),
    OpeningConvenientMinor,
    OpeningArtificialVeryStrongHand,
    OpeningWeakTwoBid,
}

pub enum OpeningBalanced15To17 {
    NoLongSuit,
    FiveCardMajor,
    ThreeCardMinor,
}

pub enum OpeningFiveCardMajor {
    SpadesGreaterThanHearts,
    SpadesEqualHearts,
    HeartsLessThanSpades,
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
        let reason = {
            if five_card_major {
                BidReason::OpeningBalanced15To17(OpeningBalanced15To17::FiveCardMajor)
            } else if three_card_minor {
                BidReason::OpeningBalanced15To17(OpeningBalanced15To17::ThreeCardMinor)
            } else {
                BidReason::OpeningBalanced15To17(OpeningBalanced15To17::NoLongSuit)
            }
        };
        bid(1, BidSuit::NoTrump, reason)
    } else if
        hcps(13, 21)
        && five_card_major
    {
        if spades > hearts {
            bid(1, BidSuit::Spades, BidReason::OpeningFiveCardMajor(OpeningFiveCardMajor::SpadesGreaterThanHearts))
        } else if spades == hearts {
            bid(1, BidSuit::Hearts, BidReason::OpeningFiveCardMajor(OpeningFiveCardMajor::SpadesEqualHearts))
        } else {
            bid(1, BidSuit::Hearts, BidReason::OpeningFiveCardMajor(OpeningFiveCardMajor::HeartsLessThanSpades))
        }
    } else if
        hcps(13, 21)
        && three_card_minor
    {
        let convenient_minor = clubs.max(diamonds) == 3;

        if diamonds > clubs {
            bid(1, BidSuit::Diamonds, BidReason::Todo)
        } else if clubs == diamonds {
            bid(1, BidSuit::Clubs, BidReason::Todo)
        } else {
            bid(1, BidSuit::Clubs, BidReason::Todo)
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
            } else if clubs > 3 && clubs >= diamonds {
                Some(BidSuit::Clubs)
            } else if diamonds > 3 {
                Some(BidSuit::Diamonds)
            } else {
                None
            }
        };
        let distributional_strength = todo!();
        let good_quick_tricks = todo!();

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
        hcps(5, 10)
    {
        // weak hand, try weak 2-bid

        let mut suits = view.suit_distributions();
        suits.sort_by_key(|(count, _suit)| *count);
        suits.reverse();

        let long_suit = {
            // Can't do a weak clubs 2-bid
            if suits[0].1 != Suit::Clubs {
                suits[0]
            } else {
                suits[1]
            }
        };

        if long_suit.0 >= 6 {
            let suit = long_suit.1.to_bid_suit();
            bid(2, suit, BidReason::OpeningWeakTwoBid)
        } else {
            pass()
        }
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

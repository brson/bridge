use std::cmp::Ordering;
use std::convert::TryInto;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub struct Deck {
    pub north: Hand,
    pub east: Hand,
    pub south: Hand,
    pub west: Hand,
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub struct Hand {
    pub cards: [Card; 13],
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub struct Card(u8);

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct AuctionState {
    pub deck: Deck,
    pub dealer: Seat,
    pub calls: Vec<PlayerCall>
}

pub struct AuctionPlayerView {
    pub hand: Hand,
    pub calls: Vec<PlayerCall>,
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone)]
pub struct PlayerCall {
    pub player: Seat,
    pub call: Call,
}

#[derive(Serialize, Deserialize)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Call {
    Bid(Bid),
    Pass,
    Double,
    Redouble,
}

#[derive(Serialize, Deserialize)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Bid {
    pub level: Level,
    pub suit: BidSuit,
}

#[derive(Serialize, Deserialize)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum BidSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    NoTrump,
}

#[derive(Serialize, Deserialize)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Serialize, Deserialize)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Level(u8);

#[derive(Serialize, Deserialize)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Seat {
    North, South, East, West
}

impl AuctionState {
    pub fn bidder_view(&self) -> AuctionPlayerView {
        AuctionPlayerView {
            hand: self.bidder_hand(),
            calls: self.calls.clone()
        }
    }

    pub fn bidder_hand(&self) -> Hand {
        match self.next_player() {
            Seat::North => self.deck.north,
            Seat::East => self.deck.east,
            Seat::South => self.deck.south,
            Seat::West => self.deck.west
        }
    }

    pub fn finished(&self) -> bool {
        maybe_next_player(&self.calls, self.dealer).is_some()
    }

    pub fn next_player(&self) -> Seat {
        maybe_next_player(&self.calls, self.dealer).expect("bidding not open")
    }
}

impl AuctionPlayerView {
    pub fn opening(&self) -> bool {
        self.calls.iter().all(|call| call.call == Call::Pass)
    }

    pub fn high_card_points(&self) -> u8 {
        let hcps = self.hand.cards.iter().map(Card::points).sum();
        assert!(hcps <= MAX_HCPS);
        hcps
    }

    pub fn balanced(&self) -> bool {
        let mut suits = self.suit_distributions()
            .map(|(count, _suit)| count);
        suits.sort();

        let balanced_dists = [
            [3, 3, 3, 4],
            [2, 3, 4, 4],
            [2, 3, 3, 5],
        ];

        balanced_dists.iter().any(|balanced| suits == *balanced)
    }

    pub fn suit_distributions(&self) -> [(u8, Suit); 4] {
        let clubs = self.hand.count_suit(Suit::Clubs);
        let diamonds = self.hand.count_suit(Suit::Diamonds);
        let hearts = self.hand.count_suit(Suit::Hearts);
        let spades = self.hand.count_suit(Suit::Spades);

        [
            (clubs, Suit::Clubs),
            (diamonds, Suit::Diamonds),
            (hearts, Suit::Hearts),
            (spades, Suit::Spades),
        ]
    }
}

fn maybe_next_player(calls: &[PlayerCall], dealer: Seat) -> Option<Seat> {
    if have_three_passes(calls) {
        return None;
    }

    if let Some(last) = calls.last().cloned() {
        Some(last.player.next())
    } else {
        Some(dealer)
    }
}

fn have_three_passes(calls: &[PlayerCall]) -> bool {
    let callcount = calls.len();
    if callcount < 3 {
        return false;
    }
    let dropcalls = callcount - 3;
    calls.iter().take(dropcalls)
        .all(|call| call.call == Call::Pass)
}

impl Seat {
    pub fn next(&self) -> Seat {
        match self {
            Seat::North => Seat::East,
            Seat::East => Seat::South,
            Seat::South => Seat::West,
            Seat::West => Seat::North,
        }
    }
}

const ACE_FACE_VALUE: u8 = 14;
const KING_FACE_VALUE: u8 = 13;
const QUEEN_FACE_VALUE: u8 = 12;
const JACK_FACE_VALUE: u8 = 11;

const ACE_POINTS: u8 = 4;
const KING_POINTS: u8 = 3;
const QUEEN_POINTS: u8 = 2;
const JACK_POINTS: u8 = 1;

pub const MAX_HCPS: u8 =
    4 * ACE_POINTS +
    4 * KING_POINTS +
    4 * QUEEN_POINTS +
    1 * JACK_POINTS;

impl Card {
    pub fn points(&self) -> u8 {
        let face_value = self.face_value();
        assert!(face_value <= ACE_FACE_VALUE);
        assert!(face_value >= 2);
        match face_value {
            ACE_FACE_VALUE => ACE_POINTS,
            KING_FACE_VALUE => KING_POINTS,
            QUEEN_FACE_VALUE => QUEEN_POINTS,
            JACK_FACE_VALUE => JACK_POINTS,
            _ => 0,
        }
    }

    pub fn face_value(&self) -> u8 {
        self.0 % 13 + 2
    }

    pub fn suite(&self) -> Suit {
        if (0..=12).contains(&self.0) {
            Suit::Clubs
        } else if (13..=25).contains(&self.0) {
            Suit::Diamonds
        } else if (26..=38).contains(&self.0) {
            Suit::Hearts
        } else if (39..=51).contains(&self.0) {
            Suit::Spades
        } else {
            unreachable!()
        }
    }
}

impl Hand {
    pub fn count_suit(&self, suit: Suit) -> u8 {
        self.cards.iter().filter(|c| c.suite() == suit)
            .count()
            .try_into().expect("u8")
    }
}

impl Suit {
    pub fn to_bid_suit(&self) -> BidSuit {
        match self {
            Suit::Clubs => BidSuit::Clubs,
            Suit::Diamonds => BidSuit::Diamonds,
            Suit::Hearts => BidSuit::Hearts,
            Suit::Spades => BidSuit::Spades,
        }
    }
}

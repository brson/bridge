use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct Card(u8);

#[derive(Copy, Clone)]
pub struct Deck {
    pub north: Hand,
    pub east: Hand,
    pub south: Hand,
    pub west: Hand,
}

#[derive(Copy, Clone)]
pub struct Hand {
    pub cards: [Card; 13],
}

#[derive(Clone)]
pub struct AuctionState {
    pub deck: Deck,
    pub dealer: Seat,
    pub calls: Vec<PlayerCall>
}

pub struct BidderView {
    pub hand: Hand,
    pub dealer: Seat,
    pub calls: Vec<PlayerCall>,
}

#[derive(Copy, Clone)]
pub struct PlayerCall {
    pub player: Seat,
    pub call: Call,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Call {
    Bid(Bid),
    Pass,
    Double,
    Redouble,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Bid {
    NoTrump(Wins),
    Spades(Wins),
    Hearts(Wins),
    Diamonds(Wins),
    Clubs(Wins),
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Wins(u8);

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Seat {
    North, South, East, West
}

impl AuctionState {
    pub fn bidder_view(&self) -> BidderView {
        BidderView {
            hand: self.bidder_hand(),
            dealer: self.dealer,
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
        self.maybe_next_player().is_some()
    }

    pub fn next_player(&self) -> Seat {
        self.maybe_next_player().expect("bidding not open")
    }

    pub fn maybe_next_player(&self) -> Option<Seat> {
        maybe_next_player(&self.calls, self.dealer)
    }
}

impl BidderView {
    pub fn next_player(&self) -> Seat {
        maybe_next_player(&self.calls, self.dealer).expect("bidding not open")
    }

    pub fn opening(&self) -> bool {
        self.calls.iter().all(|call| call.call == Call::Pass)
    }

    pub fn hcps(&self) -> u8 {
        self.hand.cards.iter().map(Card::points).sum()
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

const ACE_IDX: u8 = 14;
const KING_IDX: u8 = 13;
const QUEEN_IDX: u8 = 12;
const JACK_IDX: u8 = 11;

const ACE_POINTS: u8 = 4;
const KING_POINTS: u8 = 3;
const QUEEN_POINTS: u8 = 2;
const JACK_POINTS: u8 = 1;

impl Card {
    pub fn points(&self) -> u8 {
        assert!(self.0 <= ACE_IDX);
        assert!(self.0 >= 2);
        match self.0 {
            ACE_IDX => ACE_POINTS,
            KING_IDX => KING_POINTS,
            QUEEN_IDX => QUEEN_POINTS,
            JACK_IDX => JACK_POINTS,
            _ => 0,
        }
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

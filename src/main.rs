#![allow(unused)]

fn main() {
    println!("Hello, world!");
}



pub struct Card(u8);

pub struct Hands {
    north: Hand,
    south: Hand,
    east: Hand,
    weest: Hand,
}

pub struct Hand {
    cards: [Card; 13],
}

pub struct BidState {
    dealer: Seat,
    bids: Vec<Bid>
}

#[derive(Copy, Clone)]
pub struct Bid {
    player: Seat,
    trump: TrumpBid
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TrumpBid {
    NoTrump(Wins), Major(Wins), Minor(Wins), Pass
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Wins(u8);

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Seat {
    North, South, East, West
}

pub enum BidError {
    BiddingClosed,
    IncorrectPlayer { expected: Seat, actual: Seat },
}

pub fn check_bid(state: BidState, bid: Bid) -> Result<BidState, BidError> {
    check_bidding_still_open(&state)?;
    check_correct_player(&state, &bid)?;

    todo!();

    let mut state = state;
    state.bids.push(bid);

    Ok(state)
}

fn check_bidding_still_open(state: &BidState) -> Result<(), BidError> {
    if state.finished() {
        Err(BidError::BiddingClosed)
    } else {
        Ok(())
    }
}

fn check_correct_player(state: &BidState, bid: &Bid) -> Result<(), BidError> {
    if state.next_player() != bid.player {
        Err(BidError::IncorrectPlayer {
            expected: state.next_player(),
            actual: bid.player
        })
    } else {
        Ok(())
    }
}

impl BidState {
    fn finished(&self) -> bool {
        self.maybe_next_player().is_some()
    }

    fn next_player(&self) -> Seat {
        self.maybe_next_player().expect("bidding not open")
    }

    fn maybe_next_player(&self) -> Option<Seat> {
        if self.have_three_passes() {
            return None;
        }

        if let Some(last) = self.bids.last().cloned() {
            Some(last.player.next())
        } else {
            Some(self.dealer)
        }
    }

    fn have_three_passes(&self) -> bool {
        let bidcount = self.bids.len();
        if bidcount < 3 {
            return false;
        }
        let dropbids = bidcount - 3;
        self.bids.iter().take(dropbids)
            .all(|bid| bid.trump == TrumpBid::Pass)
    }
}

impl Seat {
    fn next(&self) -> Seat {
        match self {
            Seat::North => Seat::East,
            Seat::East => Seat::South,
            Seat::South => Seat::West,
            Seat::West => Seat::North,
        }
    }
}

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

pub struct Bid {
    player: Seat,
    wins: Wins,
    trump: Trump
}

pub enum Trump {
    NoTrump, Major, Minor,
}

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
        todo!()
    }

    fn next_player(&self) -> Seat {
        todo!()
    }
}

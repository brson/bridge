// http://www.kwbridge.com/basics.htm

#![allow(unused)]

pub mod defs;

use defs::*;

fn main() {
    println!("Hello, world!");
}


pub enum BidError {
    BiddingClosed,
    IncorrectPlayer { expected: Seat, actual: Seat },
}

pub struct BidEvaluationResult {
    state: BidState,
    bid: Bid,
    next_state: BidState,
    evaluation: Result<BidEvaluation, BidError>,
}

pub enum BidEvaluation {
    Unknown
}

pub struct SimulatedBids {
}


pub fn check_bid(state: BidState, bid: Bid) -> BidEvaluationResult {
    let result = evaluate_bid(&state, &bid);
    match result {
        Ok((next_state, evaluation)) => {
            BidEvaluationResult {
                state,
                bid,
                next_state,
                evaluation: Ok(evaluation)
            }
        }
        Err(e) => {
            let next_state = state.clone();
            BidEvaluationResult {
                state,
                bid,
                next_state,
                evaluation: Err(e)
            }
        }
    }
}

fn evaluate_bid(state: &BidState, bid: &Bid) -> Result<(BidState, BidEvaluation), BidError> {
    check_bidding_still_open(state)?;
    check_correct_player(state, bid)?;

    todo!();

    let bidder_view = state.bidder_view();
    let simulated = simulate_bid(&bidder_view);

    todo!();

    let mut state = state.clone();
    state.bids.push(*bid);

    let evaluation = BidEvaluation::Unknown;

    Ok((state, evaluation))
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

fn simulate_bid(view: &BidderView) -> SimulatedBids {
    let player = view.next_player();

    let opening = view.opening();

    if opening {
        let have_12plus_hcps = view.hcps() >= 12;
        todo!()
    } else {
        todo!()
    }
}


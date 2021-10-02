use crate::defs::*;
use crate::sim;

pub enum BidError {
    BiddingClosed,
    IncorrectPlayer { expected: Seat, actual: Seat },
}

pub struct BidEvaluationResult {
    pub state: AuctionState,
    pub call: PlayerCall,
    pub next_state: AuctionState,
    pub evaluation: Result<BidEvaluation, BidError>,
}

pub enum BidEvaluation {
    Unknown
}

pub fn check_call(state: AuctionState, call: PlayerCall) -> BidEvaluationResult {
    let result = evaluate_call(&state, &call);
    match result {
        Ok((next_state, evaluation)) => {
            BidEvaluationResult {
                state,
                call,
                next_state,
                evaluation: Ok(evaluation)
            }
        }
        Err(e) => {
            let next_state = state.clone();
            BidEvaluationResult {
                state,
                call,
                next_state,
                evaluation: Err(e)
            }
        }
    }
}

fn evaluate_call(state: &AuctionState, call: &PlayerCall) -> Result<(AuctionState, BidEvaluation), BidError> {
    check_bidding_still_open(state)?;
    check_correct_player(state, call)?;

    todo!();

    let bidder_view = state.bidder_view();
    let simulated = sim::simulate_call(&bidder_view);

    todo!();

    let mut state = state.clone();
    state.calls.push(*call);

    let evaluation = BidEvaluation::Unknown;

    Ok((state, evaluation))
}

fn check_bidding_still_open(state: &AuctionState) -> Result<(), BidError> {
    if state.finished() {
        Err(BidError::BiddingClosed)
    } else {
        Ok(())
    }
}

fn check_correct_player(state: &AuctionState, call: &PlayerCall) -> Result<(), BidError> {
    if state.next_player() != call.player {
        Err(BidError::IncorrectPlayer {
            expected: state.next_player(),
            actual: call.player
        })
    } else {
        Ok(())
    }
}

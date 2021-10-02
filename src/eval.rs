use crate::defs::*;
use crate::sim;

pub enum BidError {
    BiddingClosed,
    IncorrectPlayer { expected: Seat, actual: Seat },
}

pub struct BidEvaluationResult {
    pub state: BidState,
    pub call: PlayerCall,
    pub next_state: BidState,
    pub evaluation: Result<BidEvaluation, BidError>,
}

pub enum BidEvaluation {
    Unknown
}

pub fn check_call(state: BidState, call: PlayerCall) -> BidEvaluationResult {
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

fn evaluate_call(state: &BidState, call: &PlayerCall) -> Result<(BidState, BidEvaluation), BidError> {
    check_bidding_still_open(state)?;
    check_correct_player(state, call)?;

    todo!();

    let bidder_view = state.bidder_view();
    let simulated = sim::simulate_bid(&bidder_view);

    todo!();

    let mut state = state.clone();
    state.calls.push(*call);

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

fn check_correct_player(state: &BidState, call: &PlayerCall) -> Result<(), BidError> {
    if state.next_player() != call.player {
        Err(BidError::IncorrectPlayer {
            expected: state.next_player(),
            actual: call.player
        })
    } else {
        Ok(())
    }
}

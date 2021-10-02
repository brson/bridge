use crate::defs::*;
use crate::sim;

pub enum CallError {
    BiddingClosed,
    IncorrectPlayer { expected: Seat, actual: Seat },
}

pub struct CallEvaluationResult {
    pub state: AuctionState,
    pub call: PlayerCall,
    pub next_state: AuctionState,
    pub evaluation: Result<CallEvaluation, CallError>,
}

pub enum CallEvaluation {
    Unknown
}

pub fn check_call(state: AuctionState, call: PlayerCall) -> CallEvaluationResult {
    let result = evaluate_call(&state, &call);
    match result {
        Ok((next_state, evaluation)) => {
            CallEvaluationResult {
                state,
                call,
                next_state,
                evaluation: Ok(evaluation)
            }
        }
        Err(e) => {
            let next_state = state.clone();
            CallEvaluationResult {
                state,
                call,
                next_state,
                evaluation: Err(e)
            }
        }
    }
}

fn evaluate_call(state: &AuctionState, call: &PlayerCall) -> Result<(AuctionState, CallEvaluation), CallError> {
    check_bidding_still_open(state)?;
    check_correct_player(state, call)?;

    todo!();

    let bidder_view = state.bidder_view();
    let simulated = sim::simulate_call(&bidder_view);

    todo!();

    let mut state = state.clone();
    state.calls.push(*call);

    let evaluation = CallEvaluation::Unknown;

    Ok((state, evaluation))
}

fn check_bidding_still_open(state: &AuctionState) -> Result<(), CallError> {
    if state.finished() {
        Err(CallError::BiddingClosed)
    } else {
        Ok(())
    }
}

fn check_correct_player(state: &AuctionState, call: &PlayerCall) -> Result<(), CallError> {
    if state.next_player() != call.player {
        Err(CallError::IncorrectPlayer {
            expected: state.next_player(),
            actual: call.player
        })
    } else {
        Ok(())
    }
}

#![allow(unused)]

fn main() {
    println!("Hello, world!");
}



#[derive(Copy, Clone)]
pub struct Card(u8);

#[derive(Copy, Clone)]
pub struct Deck {
    north: Hand,
    east: Hand,
    south: Hand,
    west: Hand,
}

#[derive(Copy, Clone)]
pub struct Hand {
    cards: [Card; 13],
}

#[derive(Clone)]
pub struct BidState {
    deck: Deck,
    dealer: Seat,
    bids: Vec<Bid>
}

pub struct BidderView {
    hand: Hand,
    dealer: Seat,
    bids: Vec<Bid>,
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
        todo!()
    } else {
        todo!()
    }
}

impl BidState {
    fn bidder_view(&self) -> BidderView {
        BidderView {
            hand: self.bidder_hand(),
            dealer: self.dealer,
            bids: self.bids.clone()
        }
    }

    fn bidder_hand(&self) -> Hand {
        match self.next_player() {
            Seat::North => self.deck.north,
            Seat::East => self.deck.east,
            Seat::South => self.deck.south,
            Seat::West => self.deck.west
        }
    }

    fn finished(&self) -> bool {
        self.maybe_next_player().is_some()
    }

    fn next_player(&self) -> Seat {
        self.maybe_next_player().expect("bidding not open")
    }

    fn maybe_next_player(&self) -> Option<Seat> {
        maybe_next_player(&self.bids, self.dealer)
    }
}

impl BidderView {
    fn next_player(&self) -> Seat {
        maybe_next_player(&self.bids, self.dealer).expect("bidding not open")
    }

    fn opening(&self) -> bool {
        self.bids.iter().all(|bid| bid.trump == TrumpBid::Pass)
    }
}

fn maybe_next_player(bids: &[Bid], dealer: Seat) -> Option<Seat> {
    if have_three_passes(bids) {
        return None;
    }

    if let Some(last) = bids.last().cloned() {
        Some(last.player.next())
    } else {
        Some(dealer)
    }
}

fn have_three_passes(bids: &[Bid]) -> bool {
    let bidcount = bids.len();
    if bidcount < 3 {
        return false;
    }
    let dropbids = bidcount - 3;
    bids.iter().take(dropbids)
        .all(|bid| bid.trump == TrumpBid::Pass)
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

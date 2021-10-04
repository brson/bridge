use crate::defs::*;
use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;

pub fn random_auction(seed: u32) -> AuctionState {
    let mut rng = ChaCha8Rng::seed_from_u64(seed.into());

    let [north, east, south, west] = deal(&mut rng);

    AuctionState {
        deck: Deck {
            north,
            east,
            south,
            west,
        },
        dealer: Seat::North,
        calls: vec![],
    }
}

fn deal(rng: &mut impl Rng) -> [Hand; 4] {
    let mut deck: Vec<u8> = (0..).take(52).collect();

    let ref mut deck = deck.into_iter();

    let hands = [
        deal_one(deck),
        deal_one(deck),
        deal_one(deck),
        deal_one(deck),
    ];

    assert!(deck.next().is_none());

    hands
}

fn deal_one(deck: &mut impl Iterator<Item = u8>) -> Hand {
    let mut card = || Card::from(deck.next().expect("deck"));
    Hand {
        cards: [
            card(), card(), card(), card(),
            card(), card(), card(), card(),
            card(), card(), card(), card(),
            card()
        ]
    }
}

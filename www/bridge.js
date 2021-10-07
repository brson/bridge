console.assert(webpackBundle);
console.assert(webpackBundle.bridgeWasm);

const bridgeWasm = await webpackBundle.bridgeWasm;

bridgeWasm.init();

export function newGame() {
    return JSON.parse(bridgeWasm.new_game());
}

export function cardValueAndSuit(card) {
    return JSON.parse(bridgeWasm.card_value_and_suit(card));
}

export function nextPlayer(game) {
    return JSON.parse(bridgeWasm.next_player(JSON.stringify(game)));
}

export const CLUBS = "Clubs";
export const DIAMONDS = "Diamonds";
export const HEARTS = "Hearts";
export const SPADES = "Spades";
export const NOTRUMP = "NoTrump";

export function playBid(game, level, suit) {

    let call = {
        player: nextPlayer(game),
        call: {
            "Bid": {
                level: level,
                suit: suit
            }
        }
    };
    console.log(call);
    console.log(JSON.stringify(call));

    return JSON.parse(
        bridgeWasm.play_auction_call(
            JSON.stringify(game),
            JSON.stringify(call)
        )
    );
}

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

export function playBid(game, level, suit) {
    // todo
    console.log("todo");
}

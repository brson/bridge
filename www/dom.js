import * as bridge from "./bridge.js";
import * as utils from "./utils.js";

export const tableHandTemplate = utils.getElementAndAssert("table-hand-template");
export const tableCardTemplate = utils.getElementAndAssert("table-card-template");
export const tableNorth = utils.getElementAndAssert("table-north");
export const tableEast = utils.getElementAndAssert("table-east");
export const tableSouth = utils.getElementAndAssert("table-south");
export const tableWest = utils.getElementAndAssert("table-west");
export const auctionCallBidButton = utils.getElementAndAssert("call-bid");
export const auctionCallPassButton = utils.getElementAndAssert("call-pass");
export const auctionCallDoubleButton = utils.getElementAndAssert("call-double");
export const auctionCallRedoubleButton = utils.getElementAndAssert("call-redouble");
export const infoNextPlayerSpan = utils.getElementAndAssert("info-next-player");


export const north = "north";
export const east = "east";
export const south = "south";
export const west = "west";

export const two = "2";
export const three = "3";
export const four = "4";
export const five = "5";
export const six = "6";
export const seven = "7";
export const eight = "8";
export const nine = "9";
export const ten = "10";
export const jack = "J";
export const queen = "Q";
export const king = "K";
export const ace = "A";

export const clubs = "♣";
export const diamonds = "♦";
export const hearts = "♥";
export const spades = "♠";

export const seats = [
    north, east, south, west
];

export const ranks = [
    two,
    three, four, five, six,
    seven, eight, nine, ten,
    jack, queen, king, ace
];

export const suits = [
    clubs, diamonds, hearts, spades
];

export function setCard(seat, index, rank, suit) {
    console.assert(-1 != seats.indexOf(seat));
    console.assert(0 <= index && index <= 12);
    console.assert(-1 != ranks.indexOf(rank));
    console.assert(-1 != suits.indexOf(suit));

    let seatElt = null;

    if (seat == north) {
        seatElt = tableNorth;
    } else if (seat == east) {
        seatElt = tableEast;
    } else if (seat == south) {
        seatElt = tableSouth;
    } else if (seat == west) {
        seatElt = tableWest;
    }

    console.assert(seatElt);

    let rankElt = seatElt.querySelectorAll(".rank")[index];
    let suitElt = seatElt.querySelectorAll(".suit")[index];

    console.assert(rankElt);
    console.assert(suitElt);

    rankElt.innerText = rank;
    suitElt.innerText = suit;

    suitElt.classList.remove("suit-clubs");
    suitElt.classList.remove("suit-diamonds");
    suitElt.classList.remove("suit-hearts");
    suitElt.classList.remove("suit-spades");    

    suitElt.classList.add(`suit-${suit}`);
}

export function getAllCardButtons() {
    return document.querySelectorAll("#table .card");
}

export function updateHands(game) {
    let deck = game.auction.deck;
    updateHand(north, deck.north);
    updateHand(east, deck.east);
    updateHand(south, deck.south);
    updateHand(west, deck.west);
}

function updateHand(seat, hand) {
    let index = 0;
    for (let card of hand.cards) {
        let [rank, suit] = cardRankAndSuit(card);
        setCard(seat, index, rank, suit);
        index += 1;
    }
}

function cardRankAndSuit(card) {
    let [faceValue, suit] = bridge.cardValueAndSuit(card);

    let rank = faceValue.toString();
    if (faceValue == 11) {
        rank = "J";
    } else if (faceValue == 12) {
        rank = "Q";
    } else if (faceValue == 13) {
        rank = "K";
    } else if (faceValue == 14) {
        rank = "A";
    }

    let realSuit = null;
    if (suit == "Clubs") {
        realSuit = clubs;
    } else if (suit == "Diamonds") {
        realSuit = diamonds;
    } else if (suit == "Hearts") {
        realSuit = hearts;
    } else if (suit == "Spades") {
        realSuit = spades;
    }

    return [rank, realSuit];
}

export function getBidLevel() {
    let elt = document.querySelector('input[name="call-level"]:checked');
    if (elt != null) {
        parseInt(elt.value);
    } else {
        return 1;
    }
}

export function getBidSuit() {
    let elt = document.querySelector('input[name="call-suit"]:checked');
    if (elt != null) {
        return elt.value;
    } else {
        return clubs;
    }
}

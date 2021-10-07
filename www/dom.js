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


export const NORTH = "north";
export const EAST = "east";
export const SOUTH = "south";
export const WEST = "west";

const TWO = "2";
const THREE = "3";
const FOUR = "4";
const FIVE = "5";
const SIX = "6";
const SEVEN = "7";
const EIGHT = "8";
const NINE = "9";
const TEN = "10";
const JACK = "J";
const QUEEN = "Q";
const KING = "K";
const ACE = "A";

export const clubs = "♣";
export const diamonds = "♦";
export const hearts = "♥";
export const spades = "♠";

export const seats = [
    NORTH, EAST, SOUTH, WEST
];

const RANKS = [
    TWO,
    THREE, FOUR, FIVE, SIX,
    SEVEN, EIGHT, NINE, TEN,
    JACK, QUEEN, KING, ACE
];

export const suits = [
    clubs, diamonds, hearts, spades
];

export function setCard(seat, index, rank, suit) {
    console.assert(-1 != seats.indexOf(seat));
    console.assert(0 <= index && index <= 12);
    console.assert(-1 != RANKS.indexOf(rank));
    console.assert(-1 != suits.indexOf(suit));

    let seatElt = null;

    if (seat == NORTH) {
        seatElt = tableNorth;
    } else if (seat == EAST) {
        seatElt = tableEast;
    } else if (seat == SOUTH) {
        seatElt = tableSouth;
    } else if (seat == WEST) {
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
    updateHand(NORTH, deck.north);
    updateHand(NORTH, deck.east);
    updateHand(NORTH, deck.south);
    updateHand(NORTH, deck.west);
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

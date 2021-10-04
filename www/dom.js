import * as utils from "./utils.js";

export const tableHandTemplate = utils.getElementAndAssert("table-hand-template");
export const tableCardTemplate = utils.getElementAndAssert("table-card-template");
export const tableNorth = utils.getElementAndAssert("table-north");
export const tableEast = utils.getElementAndAssert("table-east");
export const tableSouth = utils.getElementAndAssert("table-south");
export const tableWest = utils.getElementAndAssert("table-west");



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
}

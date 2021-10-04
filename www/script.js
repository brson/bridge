import * as utils from "./utils.js";
import * as dom from "./dom.js";
import * as bridge from "./bridge.js";

async function main() {
    initializeTable();
    randomizeCards();
}

function initializeTable() {
    let seats = [
        dom.tableNorth,
        dom.tableEast,
        dom.tableSouth,
        dom.tableWest
    ];

    for (let seat of seats) {
        let newHand = dom.tableHandTemplate.content.cloneNode(true);
        let cardsRow = newHand.querySelector(".cards");
        for (let card = 0; card < 13; card++) {
            let newCard = dom.tableCardTemplate.content.cloneNode(true);
            cardsRow.appendChild(newCard);
        }
        seat.appendChild(newHand);
    }
}

function randomizeCards() {
    for (let seat of dom.seats) {
        for (let card = 0; card < 13; card++) {
            let randomRank = utils.randomElement(dom.ranks);
            let randomSuit = utils.randomElement(dom.suits);
            dom.setCard(seat, card, randomRank, randomSuit);
        }
    }
}

await main();

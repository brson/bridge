import * as utils from "./utils.js";
import * as dom from "./dom.js";
import * as bridge from "./bridge.js";

let game = null;

function main() {
    initializeTable();
    initializeEventHandlers();

    game = bridge.newGame();

    dom.updateHands(game);

    updateInfoBar(game);
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

function initializeEventHandlers() {
    
}

function updateInfoBar(game) {
    let nextPlayer = bridge.nextPlayer(game);
    dom.infoNextPlayerSpan.innerText = nextPlayer;
}

main();

console.assert(webpackBundle);
console.assert(webpackBundle.bridgeWasm);

const bridgeWasm = await webpackBundle.bridgeWasm;

bridgeWasm.init();

export function newGame() {
    return JSON.parse(bridgeWasm.new_game());
}

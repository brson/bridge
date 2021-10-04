console.assert(webpackBundle);
console.assert(webpackBundle.bridgeWasm);

const bridgeWasm = await webpackBundle.bridgeWasm;

bridgeWasm.init();

export function newGame() {
    return bridgeWasm.new_game();
}

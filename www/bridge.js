console.assert(webpackBundle);
console.assert(webpackBundle.bridgeWasm);

export const bridgeWasm = await webpackBundle.bridgeWasm;

bridgeWasm.init();

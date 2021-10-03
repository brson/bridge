import * as utils from "./utils.js";
import * as dom from "./dom.js";

console.assert(webpackBundle);
console.assert(webpackBundle.bridgeWasm);

const bridgeWasm = await webpackBundle.bridgeWasm;

bridgeWasm.init();

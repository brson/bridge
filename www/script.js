import * as utils from "./utils.js";
import * as dom from "./dom.js";

console.assert(webpackBundle);
console.log(webpackBundle);

const bridgeWasm = await webpackBundle.bridgeWasm;
console.log(bridgeWasm);



const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
    target: "web",
    mode: "development",
    experiments: {
        asyncWebAssembly: true
    },
    entry: {
        index: "./js/webpack-bundle.js"
    },
    output: {
        path: dist,
        filename: "webpack-bundle.js",
        library: "webpackBundle"
    },
    devServer: {
        contentBase: dist,
    },
    plugins: [
        /*
        new CopyPlugin([
            path.resolve(__dirname, "static")
        ]),
        */

        new WasmPackPlugin({
            crateDirectory: __dirname,
            outName: "bridge",
            /*extraArgs: "--target bundler"*/
        }),
    ]
};

const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: path.join(__dirname, '/src/main.ts'),
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'main.js',
    },
    resolve: {
        extensions: [".ts", ".js",".wasm"],
    },
    plugins: [
        new HtmlWebpackPlugin({
            inject: "head",
            title: "Rust example",
            template: path.join(__dirname, "./index.html"),
            chunks: ["Rust"]
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "./lib")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        })
    ],
    mode: 'development'
};

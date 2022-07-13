const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const commonConfig = {
    resolve: {
        extensions: [".ts", ".js", ".wasm"],
        alias: {
            // dragonBones: dragonBonesPath,
            rust: path.join(__dirname, "./lib/Rust_wasm.js"),
            worker: path.join(__dirname, "./src/worker"),
            render: path.join(__dirname, "./src/render"),
            utils: path.join(__dirname, "./src/utils"),
            structure: path.join(__dirname, "./src/structure")
        },
    },
    optimization: {
        minimize: true,
        concatenateModules: false,
        minimizer: [
            new TerserPlugin({
                sourceMap: false,
                parallel: true,
                extractComments: false,
                terserOptions: {
                    ecma: undefined,
                    warnings: false,
                    parse: {},
                    compress: {},
                    mangle: true, // Note `mangle.properties` is `false` by default.
                    module: false,
                    output: null,
                    toplevel: false,
                    nameCache: null,
                    ie8: false,
                    keep_classnames: true,
                    keep_fnames: true,
                    safari10: false,
                },
            }),
        ],
        splitChunks: {
            cacheGroups: {
                vendors: {
                    test: /node_modules/,
                    name: "vendor",
                    minChunks: 1
                }
            }
        }
    }
}

const workerConfig = Object.assign({}, commonConfig, {
    module: {
        rules: [
            { test: /\.ts$/, loader: "ts-loader", options: { allowTsInNodeModules: false }, exclude: "/node_modules/" },
        ],
    },
    entry: {
        worker: path.join(__dirname, "./src/worker/index.ts"),
    },
    output: {
        // This is required so workers are known where to be loaded from
        path: path.resolve(__dirname, "dist"),
        filename: `js/[name].js`,
        chunkFilename: `js/[name]_worker.js`,
        libraryTarget: "umd",
        globalObject: "this",
        library: "[name]",
    },
})

const renderConfig = Object.assign({}, commonConfig, {
    entry: {
        rusttest: path.join(__dirname, "./launcher.ts"),
    },
    module: {
        rules: [
            { test: /\.ts$/, loader: "ts-loader", options: { allowTsInNodeModules: false }, exclude: "/node_modules/" },
        ],
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "js/[name].js",
        chunkFilename: `js/[name]_Render.js`,
        libraryTarget: "umd",
        globalObject: "this",
        library: "[name]",
    },
    plugins: [
        new HtmlWebpackPlugin({
            inject: "head",
            title: "Rust example",
            template: path.join(__dirname, "./index.html"),
            chunks: ["rusttest"]
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
    ]
})

module.exports = [
    renderConfig, workerConfig
];

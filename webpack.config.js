const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require("copy-webpack-plugin");
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const commonConfig = {
    target: ['web', 'es5'],
    resolve: {
        extensions: [".ts", ".js", ".wasm"],
        fallback: {
            path: false,
            fs: false,
            child_process: false,
            crypto: false,
            url: false,
        },
    },
    experiments: {
        syncWebAssembly: true,
        topLevelAwait: true
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
        worker: path.join(__dirname, "./src/js/worker.js"),
    },
    output: {
        // This is required so workers are known where to be loaded from
        path: path.resolve(__dirname, "dist"),
        filename: `js/[name].js`,
        chunkFilename: `js/[name]_worker.js`,
        libraryTarget: "umd",
        globalObject: "this",
        library: "[name]",
    }
})
const renderConfig = Object.assign({}, commonConfig, {
    entry: {
        scripts: path.join(__dirname, "./src/scripts.js"),
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
            chunks: ["scripts"]
        }),
        // new WasmPackPlugin({
        //     crateDirectory: path.resolve(__dirname, "."),
        //     withTypeScript: true,
        //     outDir: path.resolve(__dirname, 'pkg'),
        //     outName: "wasm"
        // }),
        new CopyWebpackPlugin({
            patterns: [
                { from: "assets", to: "assets", toType: "dir" }
            ]
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        // new webpack.ProvidePlugin({
        //     TextDecoder: ['text-encoding', 'TextDecoder'],
        //     TextEncoder: ['text-encoding', 'TextEncoder']
        // })
    ],
    devServer: {
        static: {
            directory: path.join(__dirname, 'dist'),
        },
        compress: false,
        allowedHosts: "auto",
        port: 8080,
        devMiddleware: {
            writeToDisk: true,
        }
    }
})

module.exports = [
    renderConfig, workerConfig
];

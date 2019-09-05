const path = require("path");
const webpack = require("webpack");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const postcssPresetEnv = require("postcss-preset-env");

const devMode = process.env.NODE_ENV !== "production";

const distFolder = path.resolve(__dirname, "dist");
const srcFolder = path.resolve(__dirname, "src");

module.exports = {
    mode: devMode ? "development" : "production",
    devtool: devMode ? "inline-source-map" : undefined,
    devServer: {
        contentBase: distFolder,
        host: "0.0.0.0",
        allowedHosts: [
            ".nasso.io"
        ]
    },
    entry: {
        app: ["babel-polyfill", path.resolve(srcFolder, "index.js")]
    },
    output: {
        filename: "bundle.js",
        path: distFolder
    },
    module: {
        rules: [
            {
                test: /\.s?css$/,
                use: [
                    {
                        loader: devMode ? "style-loader" : MiniCssExtractPlugin.loader
                    },
                    {
                        loader: "css-loader",
                        options: {
                            sourceMap: true
                        }
                    },
                    {
                        loader: "postcss-loader",
                        options: {
                            ident: "postcss",
                            sourceMap: true,
                            plugins: () => [
                                postcssPresetEnv()
                            ]
                        }
                    },
                    {
                        loader: "sass-loader",
                        options: {
                            implementation: require("sass"),
                            sourceMap: true
                        }
                    }
                ]
            },
            {
                test: /\.js$/,
                exclude: /node_modules/,
                loader: "babel-loader",
                options: {
                    presets: ["@babel/preset-env"]
                }
            }
        ]
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "..", "silversnail"),

            outDir: "pkg",

            // watchDirectories: [
            //     path.join(__dirname, "..", "silversnail", "src")
            // ],

            // don't output typescript files
            extraArgs: "--no-typescript",

            forceMode: devMode ? "development" : "production"
        }),
        new CleanWebpackPlugin(),
        new HtmlWebpackPlugin({
            template: path.resolve(srcFolder, "index.html")
        }),
        new MiniCssExtractPlugin({
            filename: "[name].css",
            chunkFilename: "[id].css"
        })
    ]
}

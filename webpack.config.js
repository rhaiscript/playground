const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const VueLoaderPlugin = require('vue-loader/lib/plugin');
const GitRevisionPlugin = require('git-revision-webpack-plugin');
const webpack = require("webpack");
const gitRevisionPlugin = new GitRevisionPlugin({
  versionCommand: "describe --always --tags --dirty",
});

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js",
    globalObject: "this",
  },
  devServer: {
    contentBase: dist,
  },
  module: {
    rules: [{
      test: /\.css$/,
      use: ['style-loader', 'css-loader'],
    }, {
      test: /\.ttf$/,
      use: ['file-loader'],
    }, {
      test: /\.vue$/,
      use: ['vue-loader'],
    }],
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),

    new VueLoaderPlugin(),

    gitRevisionPlugin,

    new webpack.DefinePlugin({
      VERSION: JSON.stringify(gitRevisionPlugin.version()),
    }),
  ],
};

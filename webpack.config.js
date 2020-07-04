const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const VueLoaderPlugin = require('vue-loader/lib/plugin');
const GitRevisionPlugin = require('git-revision-webpack-plugin');
const webpack = require("webpack");
const gitRevisionPlugin = new GitRevisionPlugin({
  versionCommand: "describe --always --tags --dirty",
});

const fs = require('fs');
const crateVersion = {
  rhai: {
    version: null,
    gitHash: null,
    isCratesIo: false,
  },
};
try {
  const cargoLock = fs.readFileSync(path.resolve(__dirname, "Cargo.lock"), { encoding: "utf8" });
  // An awful RegExp to get the version of the `rhai` crate...
  const matches = /\[\[package\]\]\nname = "rhai"\nversion = "([0-9\.]+)"(?:\nsource = "([^"\n]+)"\n)?/.exec(cargoLock);
  if (matches) {
    crateVersion.rhai.version = matches[1];
    if (typeof matches[2] !== "undefined") {
      if (matches[2].startsWith("git+")) {
        const gitHashMatches = /git[^#]+#([0-9a-f]+)/.exec(matches[2]);
        if (gitHashMatches) {
          crateVersion.rhai.gitHash = gitHashMatches[1];
        }
      } else if (matches[2] === "registry+https://github.com/rust-lang/crates.io-index") {
        crateVersion.rhai.isCratesIo = true;
      }
    }
  }
} catch (ex) {
  console.warn("Failed to read Cargo.lock, skipping crate version defs", ex);
}
let rhaiVersionString = "unknown";
if (crateVersion.rhai.version !== null) {
  rhaiVersionString = crateVersion.rhai.version;
  if (crateVersion.rhai.gitHash !== null) {
    rhaiVersionString += ` (git+${crateVersion.rhai.gitHash.substr(0, 7)})`;
  } else if (crateVersion.rhai.isCratesIo) {
    rhaiVersionString += " (crates.io)";
  } else {
    rhaiVersionString += " (unknown source)";
  }
}

const dist = path.resolve(__dirname, "dist");

const worker = {
  name: "worker",
  mode: "production",
  entry: {
    worker: "./js/worker.js"
  },
  target: "webworker",
  output: {
    path: dist,
    filename: "[name].js",
    globalObject: "this",
    chunkFilename: "worker.[id].js",
    // webassemblyModuleFilename: "wasm.module.wasm",
  },
  devServer: {
    contentBase: dist,
    hot: false,
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  optimization: {
    usedExports: false,
  },
};

const web = {
  name: "app",
  dependencies: ["worker"],
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js",
    // webassemblyModuleFilename: "wasm.module.wasm",
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

    new VueLoaderPlugin(),

    gitRevisionPlugin,

    new webpack.DefinePlugin({
      VERSION: JSON.stringify(gitRevisionPlugin.version()),
      RHAI_VERSION: JSON.stringify(rhaiVersionString),
    }),
  ],
  optimization: {
    usedExports: false,
  },
};

module.exports = [worker, web];

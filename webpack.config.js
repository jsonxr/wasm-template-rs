const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');

const dist = path.resolve(__dirname, "dist");

module.exports = (env, args) => {
  const isProduction = (args.mode === 'production');
  return {
    mode: isProduction ? "production": "development",
    entry: './index.js',
    output: {
      path: dist,
      filename: isProduction ? '[name].[contenthash].js' : '[name].[hash].js',
    },
    experiments: {
      asyncWebAssembly: true,
    },
    plugins: [
      new HtmlWebpackPlugin({ template: 'index.html'}),
      new WasmPackPlugin({
        crateDirectory: __dirname,
      }),
    ]
  };
}

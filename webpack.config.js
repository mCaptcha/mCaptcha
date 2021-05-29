'use strict';
const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin');
//const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  devtool: 'inline-source-map',
  mode: 'development',
  //mode: 'production',
  entry: {
    bundle: './templates/index.ts',
    mobile: './templates/mobile.ts',
    verificationWidget: './templates/widget/index.ts',
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, './static/cache/bundle/'),
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'ts-loader',
      },

      {
        test: /\.s[ac]ss$/i,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader',
          {
            loader: 'sass-loader',
            options: {
              implementation: require('dart-sass'),
            },
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.js'],
  },

  plugins: [
    new MiniCssExtractPlugin(),
//    new WasmPackPlugin({
//      crateDirectory: __dirname,
//      outName: "pow.wasm",
//    }),
  ],
  optimization: {
    minimizer: [
      // For webpack@5 you can use the `...` syntax to extend existing minimizers (i.e. `terser-webpack-plugin`), uncomment the next line
      `...`,
      new CssMinimizerPlugin(),
    ],
  },
//  experiments: {
//    //  executeModule: true,
//    //  outputModule: true,
//    //syncWebAssembly: true,
//    //  topLevelAwait: true,
//    asyncWebAssembly: true,
//    //  layers: true,
//    //  lazyCompilation: true,
//  },
};

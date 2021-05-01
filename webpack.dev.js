const path = require('path');
const common = require('./webpack.common');
const merge = require('webpack-merge');
var HtmlWebpackPlugin = require('html-webpack-plugin');
const CleanWebpackPlugin = require('clean-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = merge(common, {
  mode: 'development',
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, 'static-assets/bundle'),
  },
  plugins: [
    new MiniCssExtractPlugin({filename: '[name].css'}),
    new CleanWebpackPlugin(),
  ],
  module: {
    rules: [
      {
        test: /\.scss$/,
        use: [
          MiniCssExtractPlugin.loader, //3. Extract css into files
          'css-loader', //2. Turns css into commonjs
          'sass-loader', //1. Turns sass into css
        ],
      },
    ],
  },
});

/*
 *  plugins: [
    new HtmlWebpackPlugin({
      filename: 'register/index.html',
      template: path.resolve(__dirname, 'output/register/', 'index.html'),
    }),

    new HtmlWebpackPlugin({
      filename: 'panel/index.html',
      template: path.resolve(__dirname, 'output/panel/', 'index.html'),
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'output/', 'index.html'),
    }),
  ],

*/

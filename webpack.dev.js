const path = require('path');
const common = require('./webpack.common');
const merge = require('webpack-merge');
var HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = merge(common, {
  mode: 'development',
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, 'static/bundle'),
  },
  module: {
    rules: [
      {
        test: /\.scss$/,
        use: [
          'style-loader', //3. Inject styles into DOM
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

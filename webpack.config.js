'use strict';
const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin');

const mobileCss = (new MiniCssExtractPlugin().options.filename = 'mobile.css');
const mainCss = (new MiniCssExtractPlugin().options.filename = 'main.css');

module.exports = {
  devtool: 'inline-source-map',
  mode: 'development',
  //mode: 'production',
  entry: {
    bundle:  './templates/index.ts',
    mobile: './templates/mobile.ts',
  },
  output: {
    filename: '[name].js',
    path: path.resolve(__dirname, './static-assets/bundle/'),
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

  plugins: [new MiniCssExtractPlugin()],
  optimization: {
    minimizer: [
      // For webpack@5 you can use the `...` syntax to extend existing minimizers (i.e. `terser-webpack-plugin`), uncomment the next line
      `...`,
      new CssMinimizerPlugin(),
    ],
  },
};

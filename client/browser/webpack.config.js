const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')

module.exports = {
  entry: './src/index.tsx',
  devtool: 'source-map',
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.json'],
    alias: {
      '@components': path.resolve(__dirname, 'src/components/')
    }
  },
  output: {
    path: path.join(__dirname, '/dist'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /nodes_modules/,
        loader: 'awesome-typescript-loader'
        /*use: {
          loader: require.resolve('awesome-typescript-loader'),
          options: {
            useBabel: true
          }
        }*/
      },
      {
        test: /\.scss$/,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader',
          /*{
            loader: 'postcss-loader',
            options: {
              config: path.resolve(__dirname, 'postcss.config.js')
            }
          },*/
          'sass-loader'
        ]
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: './public/index.html'
    }),
    new MiniCssExtractPlugin({
      filename: 'style.css'
    })
  ]
}

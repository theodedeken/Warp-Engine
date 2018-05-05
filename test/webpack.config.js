const path = require('path');

module.exports = {
  entry: {
    main: ['./tests.js']
  },
  target: 'node',
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
};
# Warp Engine ![alt text](https://travis-ci.com/TheOZoneBE/Warp-Engine.svg?branch=development)

Open source game engine written in Rust, compiled to Web Assembly. Allows for easy cross-platform game development without sacrificing too much performance. 

## Installation
This project requires rust-nightly, the wasm32-unknown-unknown target and [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen) to build.

After installing these dependencies, you can build the code using `build.sh`.

## Running tests
### Karma-Jasmine tests
To test the Javascript API a Karma-Jasmine configuration is used. To run the tests execute `test.sh`

## Running examples
After you have built the project using `build.sh`, you can navigate to the examples folder and run the examples using `npm run serve`

## License
This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
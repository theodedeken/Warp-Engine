cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/warp_engine.wasm --out-dir wasm/
cd test
npm i -g karma-cli
npm install
npm run travis
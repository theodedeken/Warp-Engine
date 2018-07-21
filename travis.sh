cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/warp_engine.wasm --out-dir wasm/
cd test
npm ci
npm run travis
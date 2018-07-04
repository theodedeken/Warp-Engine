cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/warp_engine.wasm --nodejs --out-dir wasm/
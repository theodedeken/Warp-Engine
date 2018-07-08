cargo +nightly build --target wasm32-unknown-unknown
RUST_BACKTRACE=1 wasm-bindgen target/wasm32-unknown-unknown/debug/warp_engine.wasm --out-dir wasm/
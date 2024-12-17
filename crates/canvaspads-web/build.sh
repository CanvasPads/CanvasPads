CARGO_TARGET_DIR="target" cargo build --target wasm32-unknown-unknown
wasm-bindgen ./target/wasm32-unknown-unknown/debug/canvaspads_web.wasm --target web --out-dir ../../web/src/wasm

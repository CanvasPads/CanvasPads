CARGO_TARGET_DIR="target" cargo build --target wasm32-unknown-unknown --release
wasm-bindgen ./target/wasm32-unknown-unknown/release/canvaspads_web.wasm --target web --out-dir ../../web/src/wasm
wasm-opt -Oz -o ../../web/src/wasm/canvaspads_web_bg.wasm ../../web/src/wasm/canvaspads_web_bg.wasm

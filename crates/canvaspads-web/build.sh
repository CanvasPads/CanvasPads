CARGO_TARGET_DIR="target" cargo build --target wasm32-unknown-unknown --release
wasm-opt -Oz -o ./target/wasm32-unknown-unknown/release/canvaspads_web.wasm ./target/wasm32-unknown-unknown/release/canvaspads_web.wasm
wasm-bindgen ./target/wasm32-unknown-unknown/release/canvaspads_web.wasm --target web --out-dir ../../web/src/wasm

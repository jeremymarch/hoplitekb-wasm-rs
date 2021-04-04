To build:

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --target web --no-typescript --out-dir target/wasm32-unknown-unknown/release/ target/wasm32-unknown-unknown/release/hoplitekb_wasm_rs.wasm

To build:

cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --target web --no-typescript --out-dir ~/Sites/hoplitekb-wasm-rs/ target/wasm32-unknown-unknown/release/hoplitekb_wasm_rs.wasm

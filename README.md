To build: (testing)

cargo build --target wasm32-unknown-unknown --release

if not installed:
cargo install -f wasm-bindgen-cli

wasm-bindgen --target web --no-typescript --out-dir ~/Sites/hoplitekb-wasm-rs/ target/wasm32-unknown-unknown/release/hoplitekb_wasm_rs.wasm

To build: (testing)

cargo build --target wasm32-unknown-unknown --release

if not installed:  
cargo install -f wasm-bindgen-cli

wasm-bindgen --target web --no-typescript --out-dir www/ target/wasm32-unknown-unknown/release/hoplitekb_wasm_rs.wasm

python3 -m http.server --directory www 8000

http://0.0.0.0:8000/

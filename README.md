To build and test with files in www directory:  

if not installed:  
cargo install -f wasm-bindgen-cli

1. cargo build --target wasm32-unknown-unknown --release

2. wasm-bindgen --target web --no-typescript --out-dir www/ target/wasm32-unknown-unknown/release/hoplitekb_wasm_rs.wasm

3. python3 -m http.server --directory www 8000

4. http://0.0.0.0:8000/


https://rustwasm.github.io/docs/book/introduction.html

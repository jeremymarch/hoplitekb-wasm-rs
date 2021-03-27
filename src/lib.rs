extern crate rustunicodetests;
use rustunicodetests::HGKUnicode_Mode;
use rustunicodetests::HGKDiacritics;
use rustunicodetests::toggle_diacritic_str;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[no_mangle]
pub extern fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[wasm_bindgen]
pub fn toggle(l:String, d:i32, on_only:bool, mode:i32) -> String {
	let dia = HGKDiacritics::ACUTE;
	let m = HGKUnicode_Mode::Precomposed;
	return toggle_diacritic_str(l.as_str(), dia, on_only, m);
}

#[wasm_bindgen]
pub fn reverse(s: String) -> String {
    s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

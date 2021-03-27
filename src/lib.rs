extern crate rustunicodetests;
use rustunicodetests::HGKUnicode_Mode;
use rustunicodetests::HGKDiacritics;
use rustunicodetests::toggle_diacritic_str;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
/*
#[no_mangle]
pub extern fn add(x: u32, y: u32) -> u32 {
    x + y
}
*/
#[wasm_bindgen]
pub fn toggle(l:&str, d:i32, on_only:bool, mode:i32) -> String {
	
	let dia = match d {
		1 => HGKDiacritics::ROUGH,
		2 => HGKDiacritics::SMOOTH,
		3 => HGKDiacritics::ACUTE,
		4 => HGKDiacritics::GRAVE,
		5 => HGKDiacritics::CIRCUMFLEX,
		6 => HGKDiacritics::MACRON,
		7 => HGKDiacritics::BREVE,
		8 => HGKDiacritics::IOTA_SUBSCRIPT,
		9 => HGKDiacritics::DIAERESIS,
		0 => HGKDiacritics::UNDERDOT,
		_ => return l.into()
	};

	let m = match mode {
		1 => HGKUnicode_Mode::CombiningOnly,
		2 => HGKUnicode_Mode::PrecomposedPUA,
		_ => HGKUnicode_Mode::Precomposed
	};

	return toggle_diacritic_str(l, dia, on_only, m);
}
/*
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
*/

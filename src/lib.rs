#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate alloc;
use alloc::string::*;

extern crate rustunicodetests;
use rustunicodetests::*;
//use rustunicodetests::HGKDiacritics;
use rustunicodetests::toggle_diacritic_str;
use rustunicodetests::transliterate;

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
		1 => HGK_ROUGH,
		2 => HGK_SMOOTH,
		3 => HGK_ACUTE,
		4 => HGK_GRAVE,
		5 => HGK_CIRCUMFLEX,
		6 => HGK_MACRON,
		7 => HGK_BREVE,
		8 => HGK_IOTA_SUBSCRIPT,
		9 => HGK_DIAERESIS,
		0 => HGK_UNDERDOT,
		_ => return l.into()
	};

	let m = match mode {
		1 => HGKUnicode_Mode::CombiningOnly,
		2 => HGKUnicode_Mode::PrecomposedPUA,
		_ => HGKUnicode_Mode::Precomposed
	};

	return toggle_diacritic_str(l, dia, on_only, m);
}

#[wasm_bindgen]
pub fn translit(l:&str) -> String {
	if l.chars().nth(0) != None {
		let input = l.chars().nth(0).unwrap();
		return transliterate(input as usize).to_string();
	}
	else {
		return l.to_string();
	}
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

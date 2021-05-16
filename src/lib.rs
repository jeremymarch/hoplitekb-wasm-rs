#![no_std]
#![deny(unsafe_code)]

//#[macro_use]
extern crate alloc;
use alloc::string::*;

extern crate rustunicodetests;
use rustunicodetests::*;
use rustunicodetests::toggle_diacritic_str;
use rustunicodetests::transliterate;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

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
		_ => return l.into() //return string unchanged, if d is invalid
	};

	let m = match mode {
		1 => HgkUnicodeMode::CombiningOnly,
		2 => HgkUnicodeMode::PrecomposedPUA,
		_ => HgkUnicodeMode::Precomposed
	};

	toggle_diacritic_str(l, dia, on_only, m)
}

#[wasm_bindgen]
pub fn translit(l:&str) -> String {
	if l.chars().next() != None {
		let input = l.chars().next().unwrap();
		transliterate(input as usize).to_string()
	}
	else {
		l.to_string()
	}
}

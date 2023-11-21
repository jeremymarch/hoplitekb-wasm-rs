#![no_std]
#![deny(unsafe_code)]

//#[macro_use]
extern crate alloc;
use alloc::string::*;

extern crate polytonic_greek;
use polytonic_greek::*;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[cfg(feature = "hgk_strip")]
#[wasm_bindgen]
pub fn strip_diacritics(l: &str) -> String {
    hgk_strip_diacritics(l, 0xFFFFFFFF)
}

#[cfg(feature = "hgk_convert")]
#[wasm_bindgen]
pub fn convert(l: &str, mode: i32) -> String {
    let m = match mode {
        1 => HgkUnicodeMode::CombiningOnly,
        2 => HgkUnicodeMode::PrecomposedPUA,
        _ => HgkUnicodeMode::Precomposed,
    };
    hgk_convert(l, m)
}

#[wasm_bindgen]
pub fn toggle(l: &str, d: i32, on_only: bool, mode: i32) -> String {
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
        _ => return l.into(), //return string unchanged, if d is invalid
    };

    let m = match mode {
        1 => HgkUnicodeMode::CombiningOnly,
        2 => HgkUnicodeMode::PrecomposedPUA,
        _ => HgkUnicodeMode::Precomposed,
    };

    //hgk_toggle_diacritic_str(l, dia, on_only, m)
    hgk_toggle_diacritic_str_end(l, dia, on_only, m)
}

#[wasm_bindgen]
pub fn translit(l: &str) -> String {
    if l.chars().next().is_some() {
        let input = l.chars().next().unwrap();
        hgk_transliterate(input as usize).to_string()
    } else {
        l.to_string()
    }
}

#[cfg(feature = "hgk_compare")]
#[wasm_bindgen]
pub fn compare(a: &str, b: &str, compare_type: u32) -> i32 {
    hgk_compare(a, b, compare_type)
}

#[cfg(feature = "hgk_compare_multiple_forms")]
#[wasm_bindgen]
pub fn compare_multiple_forms(str1: &str, str2: &str, ignore_duplicates: bool) -> bool {
    hgk_compare_multiple_forms(str1, str2, ignore_duplicates)
}

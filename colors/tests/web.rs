//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn spin_should_return_ice_when_coffee_and_45() {
    assert_eq!("#BCE0FF", colors::spin("#C0FFEE", 45));
}

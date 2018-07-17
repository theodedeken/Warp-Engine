#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
#![recursion_limit = "500"]

extern crate glenum_bindgen;
extern crate wasm_bindgen;
// FIXME: wasm_bindgen currently errors when trying to load exported bindings from other crates
// extern crate webgl2_bindgen;

pub mod bindings;
pub mod graphics;
pub mod math;
pub mod util;

#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
#![recursion_limit = "500"]

extern crate glenum_bind;
extern crate stdweb;
extern crate wasm_bindgen;
extern crate webgl2_bind;

pub mod graphics;
pub mod math;
pub mod util;

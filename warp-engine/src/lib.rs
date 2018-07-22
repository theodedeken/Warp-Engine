#![feature(use_extern_macros, wasm_import_module)]
#![recursion_limit = "500"]

extern crate glenum_bindgen;
extern crate wasm_bindgen;
extern crate webgl2_bindgen;

pub mod drive;
pub mod graphics;
pub mod log;
pub mod math;
pub mod util;

use std::{thread, time};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn temp_drive_test(filename: &str) {
    log::log("inside test");
    let drive = drive::Drive::new(filename);
    let vector = math::vec_d::VecD::new(vec![1.2, 2.5]);
    let mut count = 1;
    log::log("before while");
    while !drive.ready() {
        log::log("before sleep");
        thread::sleep(time::Duration::from_millis(1));
        log::log(&format!("sleeping for {} ms", count));
        count += 1;
    }
    drive.spin(vector);
}

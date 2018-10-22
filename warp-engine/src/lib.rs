extern crate wasm_bindgen;
extern crate webgl_rs;

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

use std::fs::File;
use std::io::prelude::*;

#[wasm_bindgen]
pub fn eval_test(filename: &str) {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read");
    drive::eval(contents);
}

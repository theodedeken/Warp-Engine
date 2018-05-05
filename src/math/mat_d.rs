//! Row major matrix of type `f64` and handy methods.

use wasm_bindgen::prelude::*;

/// Matrix of type `f64`.
#[wasm_bindgen]
pub struct MatD {
    width: usize,
    height: usize,
    values: Vec<f64>, 
}

#[wasm_bindgen]
impl MatD {
    pub fn new(width: usize, height: usize, values: Vec<f64>) -> MatD {
        MatD { width, height, values }
    }

    pub fn with_capacity(width: usize, height: usize) -> MatD {
        MatD { width, height, values: Vec::with_capacity(width * height) }
    }
}
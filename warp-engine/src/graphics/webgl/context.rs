//use glenum::*;
use stdweb::{__js_raw_asm, _js_impl, js, Reference};
use wasm_bindgen::prelude::*;
//use stdweb::*;
//use std::ops::Deref;
//use stdweb::unstable::TryInto;
//use stdweb::web::*;

//use stdweb::UnsafeTypedArray;

#[wasm_bindgen]
pub struct WebGLContext {
    reference: Reference,
}

impl WebGLContext {
    pub fn new() -> WebGLContext {
        let gl = js! { return document.querySelector("canvas").getContext("webgl2"); };
        WebGLContext {
            reference: gl.into_reference().unwrap(),
        }
    }

    pub fn get_reference(&self) -> &Reference {
        &self.reference
    }
}

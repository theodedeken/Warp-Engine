//use glenum::*;
use super::{buffer::Buffer, matter::Matter, program::Program, shader::Shader, texture};
use stdweb::{__js_raw_asm, _js_impl, js, Reference};
use wasm_bindgen::prelude::*;
//use stdweb::*;
//use std::ops::Deref;
//use stdweb::unstable::TryInto;
//use stdweb::web::*;

//use stdweb::UnsafeTypedArray;

//TODO create shader, matter, buffer, from context instead of own new() so we can always pass a reference to the context

#[wasm_bindgen]
pub struct WebGLContext {
    reference: Reference,
}

#[wasm_bindgen]
impl WebGLContext {
    pub fn new(selector: &str) -> WebGLContext {
        let gl = js! { return document.querySelector(@{selector}).getContext("webgl2"); };
        WebGLContext {
            reference: gl.into_reference().unwrap(),
        }
    }
}

impl WebGLContext {
    pub fn get_reference(&self) -> &Reference {
        &self.reference
    }

    pub fn clone(&self) -> WebGLContext {
        WebGLContext {
            reference: self.reference.clone(),
        }
    }
}

use super::{context::WebGLContext, matter::Matter, program::Program};
use stdweb::{Reference, __js_raw_asm, _js_impl, js};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Binding {
    context: WebGLContext,
    attrib_pointer: Reference,
}

#[wasm_bindgen]
impl Binding {
    pub fn new(
        context: &WebGLContext,
        program: Program,
        matter: Matter,
        attribute: &str,
    ) -> Binding {
        //program.use();
        matter.bind();
        let context = context.clone();
        let value = js! {
            let loc = (@{context.get_reference()}).getAttribLocation(@{program.get_reference()},@{attribute})
            return (@{context.get_reference()}).vertexAttribPointer(loc, 3, gl.FLOAT, false, 0, 0);
        };

        Binding {
            context,
            attrib_pointer: value.into_reference().expect("error: create_binding"),
        }
    }
}

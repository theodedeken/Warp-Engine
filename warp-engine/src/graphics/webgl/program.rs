use graphics::webgl::{context::WebGLContext, shader::Shader};
use stdweb::{js, Reference, __js_raw_asm, _js_impl};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Program {
    reference: Reference,
    context: WebGLContext,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

#[wasm_bindgen]
impl Program {
    pub fn new(vertex_shader: Shader, fragment_shader: Shader) -> Program {
        let context = WebGLContext::new();
        let value = js! {(@{context.get_reference()}).createProgram();};
        let reference = value.into_reference().unwrap();
        js! {@(no_return)
            (@{context.get_reference()}).attachShader(@{&reference},@{vertex_shader.get_reference()})
            (@{context.get_reference()}).attachShader(@{&reference},@{fragment_shader.get_reference()})
        };

        js! {@(no_return) (@{context.get_reference()}).linkProgram(@{&reference})};
        Program {
            reference,
            context,
            vertex_shader,
            fragment_shader,
        }
    }
}

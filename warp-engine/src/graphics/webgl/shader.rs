use super::context::Context;
use wasm_bindgen::prelude::*;
use webgl_rs::ShaderKind;
use webgl_rs::{WebGL2RenderingContext, WebGLRSShader};

//#[wasm_bindgen]
#[derive(Clone)]
pub struct Shader<'a> {
    context: &'a WebGL2RenderingContext,
    shader: WebGLRSShader<'a>,
    kind: ShaderKind,
}

//#[wasm_bindgen]
impl<'a> Shader<'a> {
    pub fn new(context: &'a Context, code: &str, kind: ShaderKind) -> Shader<'a> {
        let context = &context.wgl_context;
        let shader = context.create_shader(kind);
        shader.set_shader_source(code);
        shader.compile();
        //TODO log result of compilation
        Shader {
            context,
            shader,
            kind,
        }
    }
    pub fn shader(&self) -> &WebGLRSShader {
        &self.shader
    }
}

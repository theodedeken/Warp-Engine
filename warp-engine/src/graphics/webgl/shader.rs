use wasm_bindgen::prelude::*;
use webgl_rs::ShaderKind;
use webgl_rs::{WebGL2RenderingContext, WebGLRSShader};

//#[wasm_bindgen]
pub struct Shader<'a> {
    context: &'a WebGL2RenderingContext,
    shader: WebGLRSShader<'a>,
    kind: ShaderKind,
}

//#[wasm_bindgen]
impl<'a> Shader<'a> {
    pub fn new(context: &'a WebGL2RenderingContext, code: &str, kind: ShaderKind) -> Shader<'a> {
        let shader = context.create_shader(kind);
        shader.shader_source(code);
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

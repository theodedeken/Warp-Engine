use glenum_bindgen::ShaderKind;
use wasm_bindgen::prelude::*;
use webgl2_bindgen::{WebGL2RenderingContext, WebGLShader};

#[wasm_bindgen]
pub struct Shader {
    context: WebGL2RenderingContext,
    shader: WebGLShader,
    kind: ShaderKind,
}

#[wasm_bindgen]
impl Shader {
    pub fn new(context: WebGL2RenderingContext, code: &str, kind: ShaderKind) -> Shader {
        //let context = context.copy();
        let shader = context.create_shader(kind);
        context.shader_source(&shader, code);
        context.compile_shader(&shader);
        //TODO log result of compilation
        Shader {
            context,
            shader,
            kind,
        }
    }
}

impl Shader {
    pub fn shader(&self) -> &WebGLShader {
        &self.shader
    }
}

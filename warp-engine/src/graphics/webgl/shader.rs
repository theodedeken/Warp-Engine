use glenum_bind::ShaderKind;
use graphics::webgl::context::WebGLContext;
use stdweb::{Reference, __js_raw_asm, _js_impl, js};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Shader {
    context: WebGLContext,
    reference: Reference,
    kind: ShaderKind,
}

#[wasm_bindgen]
impl Shader {
    pub fn new(code: &str, kind: ShaderKind) -> Shader {
        let context = WebGLContext::new();
        let value = js! {
            let context = @{context.get_reference()};
            let shader = context.createShader(@{ kind as u32 });
            context.shaderSource(shader,@{ code });
            context.compileShader(shader);

            var compiled = context.getShaderParameter(shader, 0x8B81);
            console.log("Shader compiled successfully:", compiled);
            var compilationLog = context.getShaderInfoLog(shader);
            console.log("Shader compiler log:",compilationLog);
            return shader;
        };
        Shader {
            context,
            reference: value.into_reference().unwrap(),
            kind,
        }
    }
}

impl Shader {
    pub fn get_reference(&self) -> &Reference {
        &self.reference
    }
}

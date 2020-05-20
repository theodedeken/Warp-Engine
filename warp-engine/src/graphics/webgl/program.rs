use super::context::Context;
use graphics::webgl::shader::Shader;
use wasm_bindgen::prelude::*;
use webgl_rs::{WebGL2RenderingContext, WebGLRSProgram};

//#[wasm_bindgen]
#[derive(Clone)]
pub struct Program<'a> {
    context: &'a WebGL2RenderingContext,
    program: WebGLRSProgram<'a>,
    vertex_shader: Shader<'a>,
    fragment_shader: Shader<'a>,
}

//#[wasm_bindgen]
impl<'a> Program<'a> {
    pub fn new(
        context: &'a Context,
        vertex_shader: Shader<'a>,
        fragment_shader: Shader<'a>,
    ) -> Program<'a> {
        let context = &context.wgl_context;
        let program = context.create_program();
        program.attach_shader(vertex_shader.shader());
        program.attach_shader(fragment_shader.shader());
        program.link();
        Program {
            context,
            program,
            vertex_shader,
            fragment_shader,
        }
    }

    pub fn enable(&mut self) {
        self.program.enable();
    }

    pub fn get_location(&self, attribute: &str) -> u32 {
        self.program.attrib_location(attribute)
    }
}

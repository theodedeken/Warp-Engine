use graphics::webgl::shader::Shader;
use wasm_bindgen::prelude::*;
use webgl_rs::{WebGL2RenderingContext, WebGLRSProgram};

//#[wasm_bindgen]
pub struct Program<'a> {
    context: &'a WebGL2RenderingContext,
    program: WebGLRSProgram<'a>,
    vertex_shader: Shader<'a>,
    fragment_shader: Shader<'a>,
}

//#[wasm_bindgen]
impl<'a> Program<'a> {
    pub fn new(
        context: &'a WebGL2RenderingContext,
        vertex_shader: Shader<'a>,
        fragment_shader: Shader<'a>,
    ) -> Program<'a> {
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

    pub fn enable(&self) {
        self.program.enable();
    }

    pub fn get_location(&self, attribute: &str) -> u32 {
        self.program.attrib_location(attribute)
    }
}

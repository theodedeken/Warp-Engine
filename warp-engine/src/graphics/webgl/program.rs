use graphics::webgl::shader::Shader;
use wasm_bindgen::prelude::*;
use webgl_rs::{WebGL2RenderingContext, WebGLProgram};

#[wasm_bindgen]
pub struct Program {
    context: WebGL2RenderingContext,
    program: WebGLProgram,
    vertex_shader: Shader,
    fragment_shader: Shader,
}

#[wasm_bindgen]
impl Program {
    pub fn new(
        context: WebGL2RenderingContext,
        vertex_shader: Shader,
        fragment_shader: Shader,
    ) -> Program {
        let program = context.create_program();
        context.attach_shader(&program, vertex_shader.shader());
        context.attach_shader(&program, fragment_shader.shader());
        context.link_program(&program);
        Program {
            context,
            program,
            vertex_shader,
            fragment_shader,
        }
    }

    pub fn enable(&self) {
        self.context.use_program(&self.program);
    }

    pub fn get_location(&self, attribute: &str) -> u32 {
        self.context.get_attrib_location(&self.program, attribute)
    }
}

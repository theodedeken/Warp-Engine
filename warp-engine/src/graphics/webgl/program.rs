use graphics::webgl::shader::Shader;
use wasm_bindgen::prelude::*;
use webgl2_bindgen::{WebGL2RenderingContext, WebGLProgram};

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
}

extern crate rhai;
extern crate wasm_bindgen;
extern crate webgl_rs;

pub mod drive;
pub mod graphics;
pub mod log;
pub mod math;
pub mod scripting;
pub mod util;

//TEMP script
static SCRIPT: &str = r##"
let size = [800, 600];

let vertices = [-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];

let indices = [0, 1, 2];
let count = 3;

let context = get_context();

let matter = create_matter(context, vertices, indices);
true; /*
/*================ Shaders ====================*/

// Vertex shader source code
let vert_code = "#version 300 es \n in vec4 a_position; \n void main() { \n gl_Position = a_position; }";

let vert_shader = create_shader(context, vert_code, module.ShaderKind.Vertex);

//fragment shader source code
let frag_code = "#version 300 es \n precision mediump float; \n out vec4 outColor; \n void main() {    outColor = vec4(1, 0, 0.5, 1);}";
// Create fragment shader object
let frag_shader = create_shader(context, frag_code, module.ShaderKind.Fragment);

let shader_program = create_program(context, vert_shader, frag_shader);

// Use the combined shader program object
shader_program.enable();

/*======= Associating shaders to buffer objects =======*/

let matterbind = create_binding(context, shader_program, matter, "a_position")
// Enable the binding
matterbind.enable()

/*=========Drawing the triangle===========*/

// Enable the depth test
context.enable(module.Flag.DepthTest);

context.clear_color(0.5, 0.5, 0.5, 1.0);

// Set the view port
context.viewport(0, 0, size[0], size[1])


// This would ideally be in the renderloop
context.clear(module.BufferBit.Color);
context.clear(module.BufferBit.Depth);

context.draw_elements(module.Primitives.Triangles, count, module.DataType.U16, 0);
*/
"##;

use graphics::webgl::context::Context;
use wasm_bindgen::prelude::*;
use webgl_rs::rendering_context::WebGL2RenderingContext;

static mut CONTEXT: Option<Context> = None;

pub fn get_context() -> Context {
    unsafe {
        return CONTEXT.clone().unwrap();
    }
}

#[wasm_bindgen]
pub fn set_context(context: WebGL2RenderingContext) {
    unsafe {
        CONTEXT = Some(Context {
            wgl_context: context,
        });
    }
}

#[wasm_bindgen]
pub fn startup(filename: &str) {
    let mut engine = scripting::engine::RhaiEngine::new();
    engine.init();
    let result = engine.engine.eval::<bool>(SCRIPT);
    match result {
        Ok(status) => {
            if status {
                log::log("script succeeded");
            } else {
                log::log("script failed");
            }
        }
        Err(error) => {
            log::log(&format!("script errored: {}", error));
        }
    }
    /*
    let result = engine.engine.eval_file::<bool>(filename);
    match result {
        Ok(status) => {
            if status {
                log::log("script succeeded");
            } else {
                log::log("script failed");
            }
        }
        Err(error) => {
            log::log(&format!("script errored: {}", error));
        }
    }*/
}

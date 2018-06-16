use glenum_bind::{BufferKind, DrawMode};
use graphics::webgl::buffer::Buffer;
use graphics::webgl::context::WebGLContext;
use util::IntoBytes;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Matter {
    context: WebGLContext,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

#[wasm_bindgen]
impl Matter {
    pub fn new(vertices: Vec<f32>, indices: Vec<u16>) -> Matter {
        let vertex_buffer = Buffer::new(BufferKind::Array);
        vertex_buffer.load_data(&vertices.into_bytes(), DrawMode::Static);
        let index_buffer = Buffer::new(BufferKind::ElementArray);
        index_buffer.load_data(&indices.into_bytes(), DrawMode::Static);
        let context = WebGLContext::new();
        Matter {
            context,
            vertex_buffer,
            index_buffer,
        }
    }
}

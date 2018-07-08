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

impl Matter {
    pub fn new(context: &WebGLContext, vertices: Vec<f32>, indices: Vec<u16>) -> Matter {
        let vertex_buffer = Buffer::new(context, BufferKind::Array);
        vertex_buffer.load_data(&vertices.into_bytes(), DrawMode::Static);
        let index_buffer = Buffer::new(context, BufferKind::ElementArray);
        index_buffer.load_data(&indices.into_bytes(), DrawMode::Static);

        Matter {
            context: context.clone(),
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn bind(&self) {
        self.vertex_buffer.bind();
        self.index_buffer.bind();
    }
}

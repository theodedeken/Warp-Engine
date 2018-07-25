use super::buffer::Buffer;
use log::log;
use util::IntoBytes;
use wasm_bindgen::prelude::*;
use webgl_rs::WebGL2RenderingContext;
use webgl_rs::{BufferKind, DataHint};

#[wasm_bindgen]
pub struct Matter {
    context: WebGL2RenderingContext,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

#[wasm_bindgen]
impl Matter {
    pub fn new(context: WebGL2RenderingContext, vertices: Vec<f32>, indices: Vec<u16>) -> Matter {
        let vertex_buffer = Buffer::new(context.clone(), BufferKind::Array);
        vertex_buffer.load_data(vertices.into_bytes(), DataHint::StaticDraw);
        let index_buffer = Buffer::new(context.clone(), BufferKind::ElementArray);
        index_buffer.load_data(indices.into_bytes(), DataHint::StaticDraw);
        Matter {
            context,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn bind(&self) {
        self.vertex_buffer.bind();
        self.index_buffer.bind();
    }
}

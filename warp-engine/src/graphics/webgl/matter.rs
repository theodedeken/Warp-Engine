use super::buffer::Buffer;
use bindings::WebGL2RenderingContext;
use glenum_bindgen::{BufferKind, DataHint};
use util::IntoBytes;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Matter {
    context: *const WebGL2RenderingContext,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

#[wasm_bindgen]
impl Matter {
    pub fn new(context: &WebGL2RenderingContext, vertices: Vec<f32>, indices: Vec<u16>) -> Matter {
        let vertex_buffer = Buffer::new(context, BufferKind::Array);
        vertex_buffer.load_data(vertices.into_bytes(), DataHint::StaticDraw);
        let index_buffer = Buffer::new(context, BufferKind::ElementArray);
        index_buffer.load_data(indices.into_bytes(), DataHint::StaticDraw);
        //let pointer: *const WebGL2RenderingContext = &context;
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

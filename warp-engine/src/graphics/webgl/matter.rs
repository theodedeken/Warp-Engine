use super::buffer::Buffer;
use log::log;
use util::IntoBytes;
use wasm_bindgen::prelude::*;
use webgl_rs::WebGL2RenderingContext;
use webgl_rs::{BufferKind, DataHint};

//#[wasm_bindgen]
pub struct Matter<'a> {
    context: &'a WebGL2RenderingContext,
    vertex_buffer: Buffer<'a>,
    index_buffer: Buffer<'a>,
}

//#[wasm_bindgen]
impl<'a> Matter<'a> {
    pub fn new(
        context: &'a WebGL2RenderingContext,
        vertices: &Vec<f32>,
        indices: &Vec<u16>,
    ) -> Matter<'a> {
        let vertex_buffer = Buffer::new(context, BufferKind::Array);
        vertex_buffer.load_data(vertices, DataHint::StaticDraw);
        let index_buffer = Buffer::new(context, BufferKind::ElementArray);
        index_buffer.load_data(indices, DataHint::StaticDraw);
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

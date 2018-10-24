use log::log;
use wasm_bindgen::prelude::*;
use webgl_rs::data_view::Buffer as BufferView;
use webgl_rs::{BufferKind, DataHint};
use webgl_rs::{WebGL2RenderingContext, WebGLRSBuffer};

#[derive(Clone)]
pub struct Buffer<'a> {
    context: &'a WebGL2RenderingContext,
    kind: BufferKind,
    buffer: WebGLRSBuffer<'a>,
}

impl<'a> Buffer<'a> {
    pub fn new(context: &'a WebGL2RenderingContext, kind: BufferKind) -> Buffer<'a> {
        let buffer = context.create_buffer();

        Buffer {
            context,
            kind,
            buffer,
        }
    }

    pub fn load_data<B: BufferView>(&self, data: &B, draw_mode: DataHint) {
        //let length = data.len();
        self.buffer.bind(self.kind);
        self.context.buffer_data(self.kind, data, draw_mode);

        //TODO maybe find a way to bind_buffer to null to unbind
    }

    pub fn bind(&self) {
        self.buffer.bind(self.kind);
    }
}

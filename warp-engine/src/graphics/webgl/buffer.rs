use log::log;
use wasm_bindgen::prelude::*;
use webgl_rs::{BufferKind, DataHint};
use webgl_rs::{WebGL2RenderingContext, WebGLBuffer};

pub struct Buffer {
    context: WebGL2RenderingContext,
    kind: BufferKind,
    buffer: WebGLBuffer,
}

impl Buffer {
    pub fn new(context: WebGL2RenderingContext, kind: BufferKind) -> Buffer {
        let buffer = context.create_buffer();

        Buffer {
            context,
            kind,
            buffer,
        }
    }

    pub fn load_data(&self, data: Vec<u8>, draw_mode: DataHint) {
        let length = data.len();
        self.context.bind_buffer(self.kind, &self.buffer);
        self.context.buffer_data(self.kind, data, draw_mode);

        //TODO maybe find a way to bind_buffer to null to unbind
    }

    pub fn bind(&self) {
        self.context.bind_buffer(self.kind, &self.buffer);
    }
}

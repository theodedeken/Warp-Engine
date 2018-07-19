use glenum_bindgen::{BufferKind, DataHint};
use wasm_bindgen::prelude::*;
use webgl2_bindgen::{WebGL2RenderingContext, WebGLBuffer};

#[wasm_bindgen]
pub struct Buffer {
    context: *const WebGL2RenderingContext,
    kind: BufferKind,
    buffer: WebGLBuffer,
}

#[wasm_bindgen]
impl Buffer {
    pub fn new(context: &WebGL2RenderingContext, kind: BufferKind) -> Buffer {
        let buffer = context.create_buffer();

        Buffer {
            context,
            kind,
            buffer,
        }
    }

    pub fn load_data(&self, data: Vec<u8>, draw_mode: DataHint) {
        unsafe {
            let context_ref: &WebGL2RenderingContext = &*self.context;
            context_ref.bind_buffer(self.kind, &self.buffer);
            context_ref.buffer_data(self.kind, data, draw_mode);
        }

        //TODO maybe find a way to bind_buffer to null to unbind
    }

    pub fn bind(&self) {
        unsafe {
            let context_ref: &WebGL2RenderingContext = &*self.context;
            context_ref.bind_buffer(self.kind, &self.buffer);
        }
    }
}

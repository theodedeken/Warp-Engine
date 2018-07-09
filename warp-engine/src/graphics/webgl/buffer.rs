use glenum_bind::{BufferKind, DataHint};
use graphics::webgl::context::WebGLContext;
use stdweb::web::TypedArray;
use stdweb::{Reference, __js_raw_asm, _js_impl, js};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Buffer {
    context: WebGLContext,
    kind: BufferKind,
    reference: Reference,
}

#[wasm_bindgen]
impl Buffer {
    pub fn new(context: &WebGLContext, kind: BufferKind) -> Buffer {
        let context = context.clone();
        let value = js! { return (@{&context.get_reference()}).createBuffer(); };

        Buffer {
            context,
            kind,
            reference: value.into_reference().expect("error: create_buffer"),
        }
    }

    pub fn load_data(&self, data: &[u8], draw_mode: DataHint) {
        js! {
            (@{self.context.get_reference()}).bindBuffer(@{self.kind as u32},@{&self.reference});
            (@{self.context.get_reference()}).bufferData(@{self.kind as u32},@{TypedArray::from(data)}, @{draw_mode as u32});
            (@{self.context.get_reference()}).bindBuffer(@{self.kind as u32},null);
        };
    }

    pub fn bind(&self) {
        js! { (@{self.context.get_reference()}).bindBuffer(@{self.kind as u32},@{&self.reference}) };
    }
}

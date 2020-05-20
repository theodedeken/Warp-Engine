use webgl_rs::glenum::{BufferBit, DataType, Flag, Primitives};
use webgl_rs::rendering_context::WebGL2RenderingContext;

#[derive(Clone)]
pub struct Context {
    pub wgl_context: WebGL2RenderingContext,
}

impl Context {
    pub fn new(id: &str) -> Context {
        Context {
            wgl_context: WebGL2RenderingContext::new(id),
        }
    }

    pub fn enable(&mut self, cap: Flag) {
        self.wgl_context.enable(cap);
    }

    pub fn clear_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) {
        self.wgl_context.clear_color(red, green, blue, alpha);
    }

    pub fn viewport(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.wgl_context.viewport(x, y, width, height);
    }

    pub fn clear(&mut self, mask: BufferBit) {
        self.wgl_context.clear(mask)
    }

    pub fn draw_elements(
        &mut self,
        mode: Primitives,
        count: u32,
        data_type: DataType,
        offset: i64,
    ) {
        self.wgl_context
            .draw_elements(mode, count, data_type, offset);
    }
}

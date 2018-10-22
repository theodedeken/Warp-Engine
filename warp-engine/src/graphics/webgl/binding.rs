use super::{matter::Matter, program::Program};
use log::log;
use wasm_bindgen::prelude::*;
use webgl_rs::{AttributeSize, AttributeType};
use webgl_rs::{WebGL2RenderingContext, WebGLRSVertexArrayObject};

//#[wasm_bindgen]
pub struct Binding<'a> {
    context: &'a WebGL2RenderingContext,
    vao: WebGLRSVertexArrayObject<'a>,
}

//#[wasm_bindgen]
impl<'a> Binding<'a> {
    pub fn new(
        context: &'a WebGL2RenderingContext,
        program: &Program,
        matter: &Matter,
        attribute: &str,
    ) -> Binding<'a> {
        let attrib_loc = program.get_location(attribute);
        let vao = context.create_vertex_array();
        vao.bind();
        context.enable_vertex_attrib_array(attrib_loc);
        matter.bind();
        // FIXME attributesize, stride depending on matter
        context.vertex_attrib_pointer(
            attrib_loc,
            AttributeSize::Three,
            AttributeType::Float,
            false,
            0,
            0,
        );
        Binding { context, vao }
    }

    pub fn enable(&self) {
        self.vao.bind();
    }
}

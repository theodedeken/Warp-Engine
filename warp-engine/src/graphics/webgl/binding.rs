use super::{matter::Matter, program::Program};
use glenum_bindgen::{AttributeSize, AttributeType};
use log::log;
use wasm_bindgen::prelude::*;
use webgl2_bindgen::{WebGL2RenderingContext, WebGLVertexArrayObject};

#[wasm_bindgen]
pub struct Binding {
    context: WebGL2RenderingContext,
    vao: WebGLVertexArrayObject,
}

#[wasm_bindgen]
impl Binding {
    pub fn new(
        context: WebGL2RenderingContext,
        program: &Program,
        matter: &Matter,
        attribute: &str,
    ) -> Binding {
        matter.bind();
        let attrib_loc = program.get_location(attribute);
        let vao = context.create_vertex_array();
        context.bind_vertex_array(&vao);
        context.enable_vertex_attrib_array(attrib_loc);
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
        self.context.bind_vertex_array(&self.vao);
    }
}

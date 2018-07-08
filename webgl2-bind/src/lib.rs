#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method, js_name = getElementById)]
    fn get_element_by_id(this: &HTMLDocument, id: &str) -> HTMLCanvasElement;

    type HTMLCanvasElement;
    #[wasm_bindgen(method, getter)]
    fn get_context(this: &HTMLCanvasElement, context: &str) -> WebGL2RenderingContext;
}

impl WebGL2RenderingContext {
    pub fn new(id: &str) -> WebGL2RenderingContext {
        document.get_element_by_id(id).get_context("webgl2")
    }
}

#[wasm_bindgen]
extern "C" {
    type WebGL2RenderingContext;
}

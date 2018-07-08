use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method, js_name = getElementById)]
    fn get_element_by_id(this: &HTMLDocument, id: &str) -> HTMLCanvasElement;

    pub type HTMLCanvasElement;
    #[wasm_bindgen(method)]
    fn get_context(this: &HTMLCanvasElement, context: &str) -> WebGL2RenderingContext;
}

impl WebGL2RenderingContext {
    pub fn new(id: &str) -> WebGL2RenderingContext {
        document.get_element_by_id(id).get_context("webgl2")
    }
}

// WebGL2RenderingContext
#[wasm_bindgen]
extern "C" {
    /// The WebGL2RenderingContext interface provides the OpenGL ES 3.0 rendering context
    /// for the drawing surface of an HTML <canvas> element.
    pub type WebGL2RenderingContext;

    /// The `WebGLRenderingContext.canvas` property is a read-only reference to the `HTMLCanvasElement`
    /// or `OffscreenCanvas` object that is associated with the context. It might be null if it is not
    /// associated with a <canvas> element or an `OffscreenCanvas` object.
    #[wasm_bindgen(method, getter)]
    pub fn canvas(this: &WebGL2RenderingContext) -> HTMLCanvasElement;

    /// The read-only `WebGLRenderingContext.drawingBufferWidth` property represents the actual width
    /// of the current drawing buffer. It should match the width attribute of the `<canvas>` element
    /// associated with this context, but might differ if the implementation is not able to provide
    /// the requested width.
    #[wasm_bindgen(method, getter = drawingBufferWidth)]
    pub fn drawing_buffer_width(this: &WebGL2RenderingContext) -> i64;

    /// The read-only `WebGLRenderingContext.drawingBufferHeight` property represents the actual height
    /// of the current drawing buffer. It should match the height attribute of the `<canvas>` element
    /// associated with this context, but might differ if the implementation is not able to provide
    /// the requested height.
    #[wasm_bindgen(method, getter = drawingBufferHeight)]
    pub fn drawing_buffer_height(this: &WebGL2RenderingContext) -> i64;

    /// The `WebGLRenderingContext.getContextAttributes()` method returns a `WebGLContextAttributes`
    /// object that contains the actual context parameters. Might return `null`, if the context is lost.
    #[wasm_bindgen(method, js_name = getContextAttributes)]
    pub fn get_context_attributes(this: &WebGL2RenderingContext) -> WebGLContextAttributes;

    /// The WebGLRenderingContext.isContextLost() method returns a Boolean indicating whether or not
    /// the WebGL context has been lost.
    #[wasm_bindgen(method, js_name = isContextLost)]
    pub fn is_context_lost(this: &WebGL2RenderingContext) -> bool;
}

//WebGLContextAttributes
#[wasm_bindgen]
extern "C" {
    pub type WebGLContextAttributes;
}

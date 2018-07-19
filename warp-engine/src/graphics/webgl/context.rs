use webgl2_bindgen::WebGL2RenderingContext;

//note: std::ptr::read or mem::transmute seem interesting
#[derive(Clone)]
pub struct Context(WebGL2RenderingContext);

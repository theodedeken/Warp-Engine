use wasm_bindgen::prelude::*;
//TEMP
use math::vec_d::VecD;

#[wasm_bindgen(module = "./warp_drive")]
extern "C" {
    pub type Drive;

    #[wasm_bindgen(constructor)]
    pub fn new(filename: &str) -> Drive;

    #[wasm_bindgen(method, getter = ready)]
    pub fn ready(this: &Drive) -> bool;

    //TEMP arg, just for testing
    #[wasm_bindgen(method)]
    pub fn spin(this: &Drive, arg: VecD);
}

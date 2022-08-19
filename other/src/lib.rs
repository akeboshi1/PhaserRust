extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// ================= export rust to js
#[wasm_bindgen]
pub struct Foo {
    internal: i32,
}

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new(val: i32) -> Foo {
        Foo { internal: val }
    }

    pub fn get(&self) -> i32 {
        self.internal
    }

    pub fn set(&mut self, val: i32) {
        self.internal = val;
    }

    pub fn test(&mut self)->String{
        let str = format!("Foo test {:?}",self.internal);
        str
    }
}
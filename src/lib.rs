#![allow(non_snake_case)]
#[warn(unused_imports)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
pub fn action(input: &str) -> String {
    let output = if input == "" {
        "".to_string()
    } else {
        format!("Hello, {}!", input)
    };

    console_log!("Wasm in Worker says: {}",&output);

    output
}

#[wasm_bindgen]
pub fn wasm_add(num1:i32,num2:i32)-> i32 {

    let output = num1+num2;

    output
}

use std::future::Future;

fn foo() ->impl Future<Output = u8> {
    async {
        5
    }
}

#[wasm_bindgen]
pub async fn test() -> u8 {
   let x = foo().await;
   x
}

#![allow(non_snake_case)]
#[warn(unused_imports)]
#[warn(unused_must_use)]
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use js_sys::*;
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


#[wasm_bindgen]
pub async fn addThreadTest(){
        let h1 = thread::spawn(|| async{
          foo().await
        });
        h1.join().unwrap();
}


use std::io::{self, BufRead};
use std::fs::File;
use std::{thread, string};
#[wasm_bindgen]
pub fn sum_file_sync(file_path: &str) -> std::result::Result<f64,JsError> {
    let f = File::open(file_path)?;
    let reader = io::BufReader::new(f);
    let mut sum = 0.0;
    for line in reader.lines() {
        if let Ok(n) = line?.parse::<f64>() {
            println!("{}", n);
            sum += n;
        }
    }
    Ok(sum)
}

#[wasm_bindgen]
pub fn run() -> u32{
    let now = js_sys::Date::now();
    let now_date = js_sys::Date::new(&JsValue::from_f64(now));
    let x = now_date.get_milliseconds();
    x
    // let val = document.createElement("p");

    // val.set_inner_html(&format!(
    //     "Hello from Rust, it's {}:{}",
    //     now_date.get_hours(),
    //     now_date.get_minutes()
    // ));
    // document.body().append_child(val);
}
// #[wasm_bindgen]
// async fn get_from_js() -> Result<JsValue, JsValue> {
//     let promise = js_sys::Promise::resolve(&42.into());
//     let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
//     Ok(result)
// }



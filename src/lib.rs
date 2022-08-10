#![allow(non_snake_case)]
#[warn(unused_imports)]
#[warn(unused_must_use)]

extern crate wasm_bindgen;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

mod rs;

#[wasm_bindgen]
pub fn action(input: &str) -> String {
    let output = if input == "" {
        "".to_string()
    } else {
        format!("Hello, {}!", input)
    };

    log!("Wasm in Worker says: {}",&output);

    output
}

#[wasm_bindgen]
pub fn wasm_add(num1:i32,num2:i32)-> i32 {

    let output = num1+num2;

    output
}


// ================= rust future
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

// ================ thread(弃用，wasm暂时只支持单线程)
#[wasm_bindgen]
pub async fn addThreadTest(){
        let h1 = thread::spawn(move|| async{
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
// =================== js_sys
#[wasm_bindgen]
pub fn run() -> u32{
    let now = js_sys::Date::now();
    let now_date = js_sys::Date::new(&JsValue::from_f64(now));
    let x = now_date.get_milliseconds();
    x
}

// =================== Promise
#[wasm_bindgen]
pub async fn get_from_js() -> Result<JsValue, JsValue> {
    let promise = js_sys::Promise::resolve(&42.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

#[wasm_bindgen]
pub async fn my_async_test() -> Result<JsValue, JsValue> {
    // Create a promise that is ready on the next tick of the micro task queue.
    let promise = js_sys::Promise::resolve(&JsValue::from(32));
    // Convert that promise into a future and make the test wait on it.
    let x = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(x)
    // 自定义恐慌输出
    //assert_eq!(x, 32);
}

// ====================== websocket
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};


// macro_rules! log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen(start)]
pub fn start_websocket() -> Result<(), JsValue> {
    // Connect to an echo server
    let ws = WebSocket::new("wss://echo.websocket.events")?;
    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    // create callback
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            log!("message event, received arraybuffer: {:?}", abuf);
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
            // here you can for example use Serde Deserialize decode the message
            // for demo purposes we switch back to Blob-type and send off another binary message
            cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
            match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
                Ok(_) => log!("binary message successfully sent"),
                Err(err) => log!("error sending message: {:?}", err),
            }
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            log!("message event, received blob: {:?}", blob);
            // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
            let fr = web_sys::FileReader::new().unwrap();
            let fr_c = fr.clone();
            // create onLoadEnd callback
            let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                let len = array.byte_length() as usize;
                log!("Blob received {}bytes: {:?}", len, array.to_vec());
                // here you can for example use the received image/png data
            });
            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&blob).expect("blob not readable");
            onloadend_cb.forget();
        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            log!("message event, received Text: {:?}", txt);
        } else {
            log!("message event, received Unknown: {:?}", e.data());
        }
    });
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        log!("socket opened");
        match cloned_ws.send_with_str("ping") {
            Ok(_) => log!("message successfully sent"),
            Err(err) => log!("error sending message: {:?}", err),
        }
        // send off binary message
        match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
            Ok(_) => log!("binary message successfully sent"),
            Err(err) => log!("error sending message: {:?}", err),
        }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}

// ================= 通过字节数组加载图片
#[wasm_bindgen]
pub async fn loadImageByUint8Array(bytes: Uint8Array) -> Result<JsValue, JsValue>{
    let bytes: &Vec<u8> = &bytes.to_vec();
    let b = rs::image::load(bytes,0);
    let _b =Uint8Array::from(&b[..]).buffer(); 
    let promise = js_sys::Promise::resolve(&_b.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

// ================= rust内部调用通过u8数组加载图片
pub async fn loadImageByU8(bytes: &[u8]) -> Result<JsValue, JsValue>{
    let b = rs::image::load(bytes,0);
    let _b =Uint8Array::from(&b[..]).buffer(); 
    let promise = js_sys::Promise::resolve(&_b.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}







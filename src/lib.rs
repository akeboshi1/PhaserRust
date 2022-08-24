#![allow(non_snake_case)]
#[warn(unused_imports)]
#[warn(unused_must_use)]

extern crate wasm_bindgen;
extern crate serde_json;

use js_sys::{Uint8Array, Number};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
mod rs;

#[wasm_bindgen(module = "/src/js/greet.js")]
extern "C"{
    type Greet;
    fn greet(a:&str)-> String;
    #[wasm_bindgen(constructor)]
    fn new() -> Greet;
    #[wasm_bindgen(method,getter)]
    fn get_number(this: &Greet) -> i32;
    #[wasm_bindgen(method,setter)]
    fn set_number(this: &Greet, val: i32);
    #[wasm_bindgen(method)]
    fn render(this:&Greet) -> String;
}

// =================== import js to rust
#[wasm_bindgen(module = "/src/js/workerTest.js")]
extern "C"{
    type workerTest;
    fn workerTest(a:&str)-> String;
}

#[wasm_bindgen(module = "/src/render/scene/render.test.js")]
extern "C"{
    type RenderTest;
    fn renderTest(a:&str)-> String;
    #[wasm_bindgen(constructor)]
    fn new() -> RenderTest;
    #[wasm_bindgen(method)]
    fn render(this:&RenderTest)->String;
}

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
    let greet1 = Greet::new();
    greet1.set_number(33);
    log!("render {}", greet1.render());
    greet(&output.to_string());
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
use std::{thread};
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
    // from & into 互为相反作用
    let x = Number::from(64);
    // into 转换必须写明转换类型
    let y = "str";
    let z : String = y.into();
    // Create a promise that is ready on the next tick of the micro  task queue.
    let promise = js_sys::Promise::resolve(&JsValue::from(32));
    // Convert that promise into a future and make the test wait on it.
    let x = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(x)
    // 自定义恐慌输出
    //assert_eq!(x, 32);
}
// ===================== closure
#[wasm_bindgen]
extern "C"{
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;

    fn clearInterval(token: f64);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/src/js/interval.js")]
extern "C"{
    type JSInterval;
    fn setToken(a:f64)-> f64;
    #[wasm_bindgen(constructor)]
    fn new() -> JSInterval;
    #[wasm_bindgen(method,getter)]
    fn get_token(this:&JSInterval)->f64;
    #[wasm_bindgen(method,setter)]
    fn set_token(this:&JSInterval,millis:f64);
}

#[wasm_bindgen]
pub struct Interval {
    closure: Closure<dyn FnMut()>,
    token: f64,
}

impl Interval {
    pub fn new<F: 'static>(millis: u32, f: F) -> Interval
    where
        F: FnMut()
    {
        // Construct a new closure.
        let closure = Closure::new(f);

        // Pass the closure to JS, to run every n milliseconds.
        let token = setInterval(&closure, millis);

        Interval {  closure , token }
    }

}

// When the Interval is destroyed, cancel its `setInterval` timer.
// impl Drop for Interval {
//     fn drop(&mut self) {
//         clearInterval(self.token);
//     }
// }

// Keep logging "hello" every second until the resulting `Interval` is dropped.
#[wasm_bindgen]
pub fn hello() -> Interval {
    Interval::new(1000, || log("hello"))
}

// ================== 将生命周期放置在rust中管理
#[wasm_bindgen]
pub fn createInterval(val: u32,str:String) -> Interval {
    let mut count = 0;
    Interval::new(val, move|| {
        log(&format!("{}", str));
        count += 1;
    })
}

// #[wasm_bindgen]
// pub fn createInterval(val: u32,str:String) -> Interval {
//     let mut count = 0;
//     let closure = Closure::wrap(Box::new(move || {
//         log(&format!("{} {}",str,count));
//         count+=1;
//     }) as Box<dyn FnMut()>);
//     //     // Pass the closure to JS, to run every n milliseconds.
//     let token = setInterval(&closure, val);

//     Interval { closure, token }
// }


#[wasm_bindgen]
pub struct TestInterval {
    content: Interval
}

#[derive(Serialize, Deserialize)]
pub struct Example {
    pub token: f64
}

#[wasm_bindgen]
impl TestInterval {
    #[wasm_bindgen(constructor)]
    pub fn new(val: u32,str:String) -> TestInterval {
        let content = createInterval(val,str);
        // let mut jsInterval:Example = jsval.into_serde().unwrap();
        // jsInterval.token = content.token;
        setToken(content.token);
        TestInterval{ content : content }
    }

    // pub fn cancel(&mut self) {
    //     self.content.cancel();
    // }

}

// ====================== websocket
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket, XmlHttpRequest, ProgressEvent, XmlHttpRequestResponseType, DedicatedWorkerGlobalScope};


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
    let b = rs::image::image::load(bytes,0);
    let _b =Uint8Array::from(&b[..]).buffer(); 
    let promise = js_sys::Promise::resolve(&_b.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

// ================= rust内部调用通过u8数组加载图片
pub async fn loadImageByU8(bytes: &[u8]) -> Result<JsValue, JsValue>{
    let b = rs::image::image::load(bytes,0);
    let _b =Uint8Array::from(&b[..]).buffer(); 
    let promise = js_sys::Promise::resolve(&_b.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}

// ================ httprequest by url
#[wasm_bindgen]
pub async fn loadTest(url: String,f: js_sys::Function)->Result<XmlHttpRequest,JsValue> {
    let mut request = rs::xmlHttpRequest::xmlHttpPostRequest::PostRequest::new_from_default();
    request.set_header(
        "Authorization".to_string(),
        "Bearer".to_string(),
    );
    workerTest(&url.to_string());
    renderTest("loadTest");
    let promise = js_sys::Promise::resolve(&f);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    request.set_request_onload(Some(result));
    let val = rs::xmlHttpRequest::xmlHttpPostRequest::PostRequest::send(request, &url)?;
    Ok(val.request)
}

#[wasm_bindgen]
pub async fn loadTest1(url: String,f: js_sys::Function)->Result<XmlHttpRequest,JsValue> {
    let mut request = rs::xmlHttpRequest::xmlHttpPostRequest::PostRequest::new_from_default();
    request.set_header(
        "Authorization".to_string(),
        "Bearer".to_string(),
    );
    let jsValue_Url = JsValue::from_str(&url);
    log!("jsvalue {:?}",&url);
    let callResult = f.call1(&JsValue::NULL,&jsValue_Url);
    match callResult {
        Ok(u)=>{
            let t = JsValue::js_typeof(&u).as_string().unwrap();
            log!("ok {:?}",t);
            request.set_request_onload(Some(u));
        }
        Err(v)=>{
            log!("error");
            request.set_request_onerror(Some(v));
        }
    };
    let val = rs::xmlHttpRequest::xmlHttpPostRequest::PostRequest::send(request, &url)?;
    Ok(val.request)
}

// ============== serde 序列化&&反序列化&&js《-》rust传输数据
#[derive(Serialize, Deserialize)]
pub struct Element {
    name: String,
    id: String,
    parent: String,
}

#[derive(Debug)]
enum ElementFoo<'a> {
    Value(&'a str),
    Nothing,
}

#[wasm_bindgen]
pub fn wasmSerde(value:&JsValue) {
    let element:Element = value.into_serde().unwrap();
    log(&element.name); 
}

#[wasm_bindgen]
pub fn wasmSerde1(value:&JsValue) -> JsValue{
    let elements:Vec<Element> = value.into_serde().unwrap();
    let iter = elements.iter();
    let mut tempStr = String::new();
    let foos = iter.map(|val|{
        // let str = String::new();
        let str = val.id.as_str();
        tempStr+=str;
        ElementFoo::Value(str)
    }).collect::<Vec<ElementFoo>>();
   let jsStr = JsValue::from_str(&tempStr);
   jsStr
}

#[derive(Debug)]
enum Foo {
    Value(i32),
    Nothing,
}

fn main() {
    let bar = [1, 2, 3];
    let foos = bar.iter().map(|&x| Foo::Value(x)).collect::<Vec<Foo>>();
    println!("{:?}", foos);
}




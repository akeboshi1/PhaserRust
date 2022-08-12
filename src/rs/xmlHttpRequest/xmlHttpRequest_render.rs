use web_sys::{AudioContext, XmlHttpRequest, console, XmlHttpRequestEventTarget};

pub async fn httpLoadAudioContext(path: &str) -> Result<web_sys::AudioContext, wasm_bindgen::JsValue> {
    let ctx = AudioContext::new();
    let request = XmlHttpRequest::new().unwrap();
    // "https://firebasestorage.googleapis.com/v0/b/podstetheedata.appspot.com/o/human_samples%2F-Lsxlh74yy4ASUohCFEA.wav?alt=media&token=6088e994-73b6-47a4-bc0d-a1090cb3b288");
//    let response_type = from_js_value(ArrayBuffer);
    request.set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
    /*let closure = Closure::wrap(Box::new(move |x|{
        console::log_1(&x.into())
    }) as Box<dyn FnMut(_)>);*/
    request.onload();
    // &XmlHttpRequestEventTarget::onload(request.onload());
    request.open("GET", path);
    let results = request.send();
    match results {
        Err(e) => println!("nothing {:?}", e),
        Ok(res) => {
            console::log_1(&"Request sent".into());
        }
    }
    /*  match request.onload() {
          None => println!("nothing..."),
          Some(f) => {
              println!("?? {:?}", f);
          }
      }*/
    ctx
}
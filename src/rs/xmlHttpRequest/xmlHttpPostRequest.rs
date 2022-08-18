use std::collections::HashMap;

use js_sys::Uint8Array;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Blob, ErrorEvent, FormData, ProgressEvent, XmlHttpRequest, XmlHttpRequestResponseType};
use crate::log;

enum PostRequestContent {
    Form(FormData),
    Blob(Blob),
}

pub struct PostRequest {
    upload_onabort: Option<JsValue>,
    upload_onerror: Option<JsValue>,
    upload_onload: Option<JsValue>,
    upload_onloadend: Option<JsValue>,
    upload_onloadstart: Option<JsValue>,
    upload_on_progress: Option<JsValue>,
    upload_ontimeout: Option<JsValue>,
    request_onabort: Option<JsValue>,
    request_onerror: Option<JsValue>,
    request_onload: Option<JsValue>,
    request_onloadend: Option<JsValue>,
    request_onloadstart: Option<JsValue>,
    request_on_progress: Option<JsValue>,
    request_ontimeout: Option<JsValue>,
    _async: bool,
    content: Option<PostRequestContent>,
    headers: HashMap<String, String>,
}

impl PostRequest {
    // pub fn new_from_form(form_id: &str) -> Self {
    //     let window = web_sys::window().expect("Fatal: No window found!");
    //     let document = window.document().expect("Fatal: No document in window");

    //     let form = document
    //         .get_element_by_id(form_id)
    //         .unwrap_or_else(|| panic!("Could not find the form specified: {}", form_id));

    //     let form_data = FormData::new_with_form(&form.dyn_into().unwrap_or_else(|_| {
    //         panic!(
    //             "The id provided (\"{}\") did not point to an html form!",
    //             form_id
    //         )
    //     }))
    //     .unwrap_or_else(|_| panic!("Could not extract form with id \"{}\"!", form_id));

    //     PostRequest {
    //         content: Some(PostRequestContent::Form(form_data)),
    //         ..Default::default()
    //     }
    // }

    pub fn new_from_default()->Self {
        PostRequest {
            ..Default::default()
        }
    }

    pub fn new_from_blob(blob: Vec<u8>, mime_type: &str) -> Self {
        let uint8arr: Uint8Array = blob.as_slice().into();
        let blob = Blob::new_with_u8_array_sequence_and_options(
            &uint8arr,
            web_sys::BlobPropertyBag::new().type_(mime_type),
        )
        .unwrap();
        PostRequest {
            content: Some(PostRequestContent::Blob(blob)),
            ..Default::default()
        }
    }

    pub fn set_upload_onabort(&mut self, closure: Option<Box<dyn Fn()>>) {
        let callback = closure.map(|c| Closure::<dyn Fn()>::wrap(c).into_js_value());
        self.upload_onabort = callback;
    }

    pub fn set_upload_onerror(&mut self, closure: Option<Box<dyn Fn(ErrorEvent)>>) {
        let callback = closure.map(|c| Closure::<dyn Fn(ErrorEvent)>::wrap(c).into_js_value());
        self.upload_onerror = callback;
    }

    pub fn set_upload_onload(&mut self, closure: Option<Box<dyn Fn(ProgressEvent)>>) {
        let callback = closure.map(|c| Closure::<dyn Fn(ProgressEvent)>::wrap(c).into_js_value());
        self.upload_onload = callback;
    }

    pub fn set_upload_onloadstart(&mut self, closure: Option<Box<dyn Fn()>>) {
        let callback = closure.map(|c| Closure::<dyn Fn()>::wrap(c).into_js_value());
        self.upload_onloadstart = callback;
    }

    pub fn set_upload_onloadend(&mut self, closure: Option<Box<dyn Fn()>>) {
        let callback = closure.map(|c| Closure::<dyn Fn()>::wrap(c).into_js_value());
        self.upload_onloadend = callback;
    }

    pub fn set_upload_onprogress(&mut self, closure: Option<Box<dyn Fn(ProgressEvent)>>) {
        let callback = closure.map(|c| Closure::<dyn Fn(ProgressEvent)>::wrap(c).into_js_value());
        self.upload_on_progress = callback;
    }

    pub fn set_request_onabort(&mut self, closure: Option<Box<dyn Fn()>>) {
        let callback = closure.map(|c| Closure::<dyn Fn()>::wrap(c).into_js_value());
        self.request_onabort = callback;
    }

    pub fn set_request_onerror(&mut self, fun:Option<JsValue>) {
        // let callback = closure.map(|c| Closure::<dyn Fn(ErrorEvent)>::wrap(c).into_js_value());
        self.request_onerror = fun;
    }

    pub fn set_request_onload(&mut self, fun:Option<JsValue>) {
       //let callback = closure.map(|c| Closure::<dyn Fn(ProgressEvent)>::wrap(c).into_js_value());
        self.request_onload = fun;
    }

    pub fn set_request_onloadstart(&mut self, closure: Option<Box<dyn Fn()>>) {
        let callback = closure.map(|c| Closure::<dyn Fn()>::wrap(c).into_js_value());
        self.request_onloadstart = callback;
    }

    pub fn set_request_onloadend(&mut self, closure: Option<Box<dyn Fn()>>) {
        let callback = closure.map(|c| Closure::<dyn Fn()>::wrap(c).into_js_value());
        self.request_onloadstart = callback;
    }

    pub fn set_request_onprogress(&mut self, closure: Option<Box<dyn Fn(ProgressEvent)>>) {
        let callback = closure.map(|c| Closure::<dyn Fn(ProgressEvent)>::wrap(c).into_js_value());
        self.request_on_progress = callback;
    }

    pub fn set_async(&mut self, _async: bool) {
        self._async = _async;
    }

    pub fn send(self, url: &str) -> Result<SendRequest, JsValue> {
        let request = self.get_request(url);
        // self.set_upload_callbacks(&request);
        self.set_request_callbacks(&request);

        self.set_headers(&request)?;

        let result: SendRequest = self.send_request(request)?.into();
        Ok(result)
    }

    pub fn set_header(&mut self, header: String, value: String) {
        self.headers.insert(header, value);
    }

    fn send_request(self, request: XmlHttpRequest) -> Result<XmlHttpRequest, JsValue> {
        let result = match &self.content {
            Some(content) => match content {
                PostRequestContent::Form(form) => request.send_with_opt_form_data(Some(form)),
                PostRequestContent::Blob(blob) => request.send_with_opt_blob(Some(blob)),
            },
            None => request.send(),
        };

        match result {
            Ok(_) => Ok(request),
            Err(error) => Err(error),
        }
    }

    fn get_request(&self, url: &str) -> XmlHttpRequest {
        let request = XmlHttpRequest::new().expect("Could not create request!");
        request
            .open_with_async("GET", url, self._async)
            .expect("Could not open request");
        request
    }

    fn set_request_callbacks(&self, request: &XmlHttpRequest) {
        log!("set_request_callbacks");
        if let Some(callback) = &self.request_onabort {
            request.set_onabort(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.request_onerror {
            request.set_onerror(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.request_onload {
            log!("Request onload{:?}", request);
            request.set_onload(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.request_onloadend {
            request.set_onloadend(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.request_onloadstart {
            request.set_onloadstart(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.request_on_progress {
            request.set_onprogress(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.request_ontimeout {
            request.set_ontimeout(Some(callback.as_ref().unchecked_ref()));
        }
    }

    fn set_upload_callbacks(&self, request: &XmlHttpRequest) {
        let upload = request
            .upload()
            .expect("Could not fetch upload field of xmlhttprequest");
        if let Some(callback) = &self.upload_onabort {
            upload.set_onabort(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.upload_onerror {
            upload.set_onerror(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.upload_onload {
            upload.set_onload(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.upload_onloadend {
            upload.set_onloadend(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.upload_onloadstart {
            upload.set_onloadstart(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.upload_on_progress {
            upload.set_onprogress(Some(callback.as_ref().unchecked_ref()));
        }
        if let Some(callback) = &self.upload_ontimeout {
            upload.set_ontimeout(Some(callback.as_ref().unchecked_ref()));
        }
    }

    fn set_headers(&self, request: &XmlHttpRequest) -> Result<(), JsValue> {
        for (key, value) in &self.headers {
            request.set_request_header(key, value)?;
        }
        Ok(())
    }
}

impl Default for PostRequest {
    fn default() -> Self {
        Self {
            _async: true,
            content: None,
            upload_onabort: None,
            upload_onerror: None,
            upload_onload: None,
            upload_onloadend: None,
            upload_onloadstart: None,
            upload_on_progress: None,
            upload_ontimeout: None,
            request_onload: None,
            request_onabort: None,
            request_onerror: None,
            request_onloadend: None,
            request_onloadstart: None,
            request_on_progress: None,
            request_ontimeout: None,
            headers: Default::default(),
        }
    }
}

pub struct SendRequest {
    pub request: XmlHttpRequest,
}

impl SendRequest {
    pub fn abort(self) {
        self.request
            .abort()
            .expect("tried to abort request which could not be aborted");
    }

    pub fn status(&self) -> u16 {
        self.request
            .status()
            .expect("Could not fetch request status")
    }

    pub fn ready_state(&self) -> u16 {
        self.request.ready_state()
    }
}

impl From<XmlHttpRequest> for SendRequest {
    fn from(request: XmlHttpRequest) -> Self {
        Self { request }
    }
}
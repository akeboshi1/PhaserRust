extern crate wasm_bindgen;
use std::u8;

use web_sys::{ErrorEvent,MessageEvent,WebSocket};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use js_sys::{ArrayBuffer, Uint8Array};

#[repr(u16)]
#[derive(FromPrimitive)]
enum ReadyState {
    CONNECTING = 0,
    OPEN = 1,
    CLOSING = 2,
    CLOSED = 3,
}

pub struct Connection {
    socket:Option<WebSocket>
}

impl Connection {


    pub fn new()->Self{
        Connection{
            socket:None
        }
    }

    pub fn get_socket(&self)-> &WebSocket{
        self.socket.as_ref().unwrap()
    }

    pub fn set_socket(&mut self,socket:WebSocket){
        self.socket = Some(socket)
    }

    pub fn start_connect(&mut self,url:&str)-> Result<(), JsValue>{
        let socket = match WebSocket::new(url) {
            Ok(socket)  => socket,
            Err(e) => return Err(e),
        };
        self.socket = Some(socket);
        self.socket.as_ref().unwrap().set_binary_type(web_sys::BinaryType::Arraybuffer);
        Ok(())
    }
    
    pub fn close_connect(&self)->Result<(),JsValue>{
        let socket = self.get_socket();
        let ready_state = socket.ready_state();
        match FromPrimitive::from_u16(ready_state) {
            Some(ReadyState::CLOSING) => {
                Err(JsValue::from_str("closing"))
            },
            Some(ReadyState::CLOSED) => {
                Err(JsValue::from_str("closed"))
            },
            Some(ReadyState::OPEN) => {
                socket.close()?;
                Ok(())
            },
            Some(ReadyState::CONNECTING) =>{
                socket.close()?;
                Ok(())
            }
            None => {
                socket.close()?;
                Ok(())
            },
            
        }
    }

    pub fn send(&self,data:JsValue) -> Result<(),JsValue> {
        let socket:&WebSocket = self.get_socket();
        // let bytes = JsValue::from
        // let bytes: &Vec<u8> = &bytes.to_vec();
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e:MessageEvent|{
            if let Ok(buf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
               // todo deserialize
            }
        });
        socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
            // todo socket error
        });
        socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
   
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            // todo socket open
        });
        socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        Ok(())
    }
}



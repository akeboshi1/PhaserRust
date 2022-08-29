use web_sys::{ErrorEvent, MessageEvent, WebSocket};
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
pub struct connection {
    socket:Option<WebSocket>
}

impl connection {


    pub fn new(&self)->Self{
        connection{
            socket:None
        }
    }

    pub fn get_socket(&self)-> &WebSocket{
        self.socket.as_ref().unwrap()
    }

    pub fn set_socket(&mut self,socket:WebSocket){
        self.socket = Some(socket)
    }

    pub fn startConnect(&mut self,url:&str)-> Result<(), JsValue>{
        let socket = match WebSocket::new(url) {
            Ok(socket)  => socket,
            Err(e) => return Err(e),
        };
        self.socket = Some(socket);
        self.socket.as_ref().unwrap().set_binary_type(web_sys::BinaryType::Arraybuffer);
        Ok(())
    }
    
    pub fn closeConnect(&self){
        
    }
}



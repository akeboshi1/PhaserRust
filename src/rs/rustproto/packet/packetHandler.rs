use std::collections::HashMap;

use js_sys::ArrayBuffer;
use wasm_bindgen::JsValue;

use super::packet::Packet;
pub trait PacketListener {
    fn on_packet_arrived(&mut self,opcode:u8,data:ArrayBuffer)->bool;
    fn add_handler_fun(&mut self,opcode:u8,callback:js_sys::Function);
}

pub struct PacketHandler{
    opcode_hashmap:HashMap<u8,js_sys::Function>,
}

impl PacketListener for PacketHandler{
    fn on_packet_arrived(&mut self,opcode:u8,data: ArrayBuffer)->bool{
        if self.opcode_hashmap.is_empty() == true {
            return false
        }

        let call_back_option = self.opcode_hashmap.get(
            &opcode
         );
         if call_back_option == None {
            let call_back:& js_sys::Function = call_back_option.unwrap();
            call_back.call1(&JsValue::NULL,&JsValue::from(data));
            // call_back.call(this, packet);
            // console.info(`Finish handle OPCODE: 0x${packet.opcode.toString(16)} / ${packet.opcode.toString()}`);
            return true;
         }
        false
    }

    fn add_handler_fun(&mut self,opcode: u8,callback:js_sys::Function){
        if self.opcode_hashmap.is_empty() == true {
            self.opcode_hashmap = HashMap::new();
        }
        self.opcode_hashmap.insert(opcode, callback);
    }
}

// 密封特性
pub(crate) mod private {

    #[doc(hidden)]
    pub trait FooPrivate<Arg> {
        fn foo(&self, arg: Arg);
    }

}

pub trait Foo<Arg>: private::FooPrivate<Arg> {

    /* other public methods */

}
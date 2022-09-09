
use std::string;

use js_sys::{ArrayBuffer, Uint8Array};   
use num_traits::FromPrimitive;
use wasm_bindgen::JsValue;
use crate::log;

use super::{packetHeader::{HEAD_BYTES_SIZE, PacketHeader}, packet::PacketTrait};
struct ProtoClass {
    
}
pub struct PBPacket{
    // ==== universal
    header : PacketHeader,
    length : u32,
    opcode : u32,
    buf : Vec<u8>,
    // ==== pbpacket
    opcode_str : String,
    pbClass : JsValue
}

impl PacketTrait for PBPacket{
    fn new()-> Self {
        PBPacket { 
            header: PacketHeader::new(),
            length: 0,
            opcode: 0,
            buf: vec![],
            opcode_str: String::from(""),
            pbClass:JsValue::as_any
        }
    }
    fn init(&mut self, opcode: u32, param:u32) {
        self.header = PacketHeader::new();
        self.header.set_opcode(opcode);
        self.header.set_param(param);
        self.opcode = opcode;
        PBPacket::reflection();
    }

    fn contentDecode(&mut self) {
        PBPacket::reflection();
        // todo refection
    }

    fn contentEncode(&mut self) {
        
    }
   

}

impl PBPacket {

    // static function
   pub fn create(data:ArrayBuffer)-> PBPacket {
        let packet = PBPacket::new();
        packet.set_up_from_arraybuffer(data);
        packet
   }
 
   pub fn reflection(){
       // todo reflection
   }
   pub fn set_up_from_arraybuffer(&mut self,data:ArrayBuffer) {
       let head_bytes_size = usize::from_u32(HEAD_BYTES_SIZE).unwrap();
       let byte_len = usize::from_u32(data.byte_length()).unwrap();
       if byte_len < head_bytes_size {
           log!("Packet is smaller than header size. {:?}","error");
           return;
       }
       let mut clone_buf = Uint8Array::new(&data).to_vec();
       // 切除头部长度的数据
       let body_buf = clone_buf.split_off(head_bytes_size);
       let head_buf = clone_buf;

       self.header.un_pack(&head_buf);
       
       let content_len = usize::from_u32(self.header.get_blen()).unwrap();

       // 切除body长度的数据
       let end_buf = body_buf.split_off(content_len - head_bytes_size);
       self.buf = clone_buf;
       
       self.contentDecode();
   }
   pub fn toString(&self) -> String{
       let str = format!("{:?}" , format_args!("OP[{}] {} - Content_Len: {:?}",self.opcode,self.opcode_str,self.buf.len()));
       String::from(str)
   }
}
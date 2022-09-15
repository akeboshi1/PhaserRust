use crate::{log};
use super::packetHeader::{PacketHeader, HEAD_BYTES_SIZE};
use num_traits::FromPrimitive;

pub trait PacketTrait {
    fn new()-> Self;
    fn init(&mut self, opcode: u32, param:u32);
    fn contentEncode(&mut self);
    fn contentDecode(&mut self);
}
pub struct Packet {
    header : PacketHeader,
    length : u32,
    opcode : u32,
    buf : Vec<u8>,
}

impl PacketTrait for Packet{
    fn new() -> Self {
        Packet { 
            header: PacketHeader::new(),
            length: 0,
            opcode: 0,
            buf: vec![]
        }
    }
    fn init(&mut self, opcode: u32, param:u32) {
        self.header = PacketHeader::new();
        self.header.set_opcode(opcode);
        self.header.set_param(param);
        self.opcode = opcode;
    }  
    fn contentDecode(&mut self) {}
    fn contentEncode(&mut self) {}
}

impl Packet {
    pub fn serialization(&mut self) -> Vec<u8>{
        let self_buf = self.buf.clone();
        let buf_len = self_buf.len();
        // init header
        self.header.set_blen(u32::from_usize(buf_len).unwrap());
        self.header.pack();
        let head_buf = self.header.get_buf().clone();
        // init out buffer
        let buf:Vec<u8> = [head_buf, self_buf].concat();
        buf
    }

    pub fn deserialization(&mut self,buf:&mut [u8]) -> bool{
        let head_bytes_size = usize::from_u32(HEAD_BYTES_SIZE).unwrap();
        if buf.len() < head_bytes_size {
            log!("Packet is smaller than header size.{:?}","error");
            return false;
        }
      
        let pos = self.header.un_pack(buf);

        self.buf = buf[0..usize::from_u32(pos).unwrap()].to_vec();
        
        self.length = pos;
        false
    }

    pub fn get_buf(&self)->&Vec<u8>{
        &self.buf
    }
    
    pub fn get_len(&self)->u32{
        self.length
    }

}
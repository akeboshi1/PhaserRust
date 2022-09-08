use crate::{log};
use super::packetHeader::{PacketHeader, PacketHeaderKey, HEAD_BYTES_SIZE};
use num_traits::FromPrimitive;
use std::{cmp};
pub struct Packet {
    header : PacketHeader,
    length : u8,
    opcode : u8,
    buf : Vec<u8>,
}

impl Packet {
    pub fn new() -> Self {
        Packet { 
            header: PacketHeader::new(),
            length: 0,
            opcode: 0,
            buf: vec![]
        }
    }

    pub fn serialization(&mut self) -> Vec<u8>{
        let buf:Vec<u8> ;
        self.mkHead();
        let head_buf = self.header.get_buf();
        let head_bytes_size = usize::from_u32(HEAD_BYTES_SIZE).unwrap();
        let mut head_buf_available = &mut head_buf[head_bytes_size..];
        let len = cmp::min(head_buf_available.len(), self.buf.len());
        head_buf_available[..len].copy_from_slice(&self.buf[..len]);
        self.header.update_offset(u32::from_usize(len).unwrap());
        buf = head_buf.clone();
        buf.split_at(head_bytes_size + len);
        buf
    }

    pub fn deserialization(&self,buf:&[u8]) -> bool{
        let head_bytes_size = usize::from_u32(HEAD_BYTES_SIZE).unwrap();
        if buf.len() < head_bytes_size {
            log!("Packet is smaller than header size.{:?}","error");
            return false;
        }
      
        let pos = self.header.un_pack(buf);
      
        let content_len = self.header.get_blen();
      
        self.buf = buf[0..pos].to_vec();

        false
    }

    pub fn init(&self, opcode: u32, param:u32) {
        self.header = PacketHeader::new();
        self.header.set_opcode(opcode);
        self.header.set_param(param);
    }

    fn mkHead(&mut self) -> PacketHeader{
        self.header.set_blen(u32::from_usize(self.buf.len()).unwrap());
        let head_buf = self.header.pack();
        let head_len = self.header.get_buf().len();
        self.header.set_buf(head_buf);
        self.header
    }

    

}
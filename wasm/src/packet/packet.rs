use crate::{utils::log, log};

use super::packetHeader::{PacketHeader, PacketHeaderKey, HEAD_BYTES_SIZE};
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use std::{cmp, error, fmt, io, ptr};
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

    pub fn serialization(&self) -> Vec<u8>{
        let buf:Vec<u8> ;
        self.mkHead();
        let head_buf = self.header.get_buf();
        let mut head_buf_available = &mut head_buf[HEAD_BYTES_SIZE..];
        let len = cmp::min(head_buf_available.len(), self.buf.len());
        head_buf_available[..len].copy_from_slice(&self.buf[..len]);
        self.header.update_offset(len);
        buf = head_buf.clone();
        buf.split_at(HEAD_BYTES_SIZE+len);
        buf
    }

    pub fn deserialization(&self,buf:&[u8]) -> bool{
        if buf.len() < HEAD_BYTES_SIZE {
            log!("Packet is smaller than header size.{:?}","error");
            return false;
        }
      
        let pos = self.header.un_pack(buf);
      
        let content_len = self.header.get_params(PacketHeaderKey::BLEN);
      
        self.buf = Buffer.alloc(content_len);
      
        buf.copy(this.mContentBuf, 0, pos);

        false
    }

    pub fn init(&self, opcode: usize, param:usize) {
        self.header = PacketHeader::new();
        self.header.set_params(PacketHeaderKey::OPCODE, opcode);
        self.header.set_params(PacketHeaderKey::PARAM, param);
    }

    fn mkHead(&self) -> PacketHeader{
        self.header.set_params(PacketHeaderKey::BLEN, self.buf.len());
        let head_buf = self.header.pack();
        let head_len = self.header.get_buf().len();
        self.header.set_buf(head_buf);
        self.header
    }

    

}
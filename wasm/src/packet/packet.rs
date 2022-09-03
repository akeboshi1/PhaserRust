extern crate buf_redux;
extern crate byteorder; // 1.2.7
use crate::log;
use std::cmp;
use byteorder::ByteOrder;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

const S : u16 = 83;// utils::stringutil::get_char_code(&"S"); 83
const D : u16 = 68;// utils::stringutil::get_char_code(&"D"); 68
// const C : u8 = 67;// utils::stringutil::get_char_code(&"C"); 67
const PACKET_MAGIC_D:u16 = (S << 8) | D; //'DS'   default=not zip
// const PACKET_MAGIC_C:u8 = (S << 8) | C; //'CS'    zip
pub const HEAD_BYTES_SIZE:usize = 2 + 4 + 4 + 4 + 4 + 8;


#[repr(u8)]
#[derive(FromPrimitive)]
pub enum PacketHeaderKey {
    OPCODE = 0,
    PARAM = 1,
    TIMESTAMP = 2,
    BLEN = 3,
    MAGIC = 4,
}
/**
   * // --- Header structure --- \\
   * +------------------------------------------------------------+
   * |    magic   +   uint16  +   2Bytes  +       zip or not      |
   * +------------------------------------------------------------+
   * |  body_len  +   uint32  +   2Bytes  +   Packet body length  |
   * +------------------------------------------------------------+
   * |    opcode  +   uint32  +   4Bytes  +    operation code     |
   * +------------------------------------------------------------+
   * |    uuid    +   uint32  +   4Bytes  +  Connection SessionID |
   * +------------------------------------------------------------+
   * |    param   +   uint32  +   4Bytes  +   parameter of packet |
   * +------------------------------------------------------------+
   * |    timestamp   +   double  +   8Bytes  +   parameter of packet |
   * +------------------------------------------------------------+
   */

// const OPCODE_REGXP = Regex::new(r"/^_(OP_.+)/").unwrap();
pub struct Packet {
    // ==== commn
    opcode:u8,
    // ==== packetheader
    head_len :usize,
    body_len:u8,
    head_param:u8,
    head_uuid:u8,
    head_time_stamp:u32,
    head_magic:u16,
    head_offset:u8,
    head_buf:Vec<u8>,
    // ==== packet
    buf : Vec<u8>,
}

impl Packet {
    pub fn new() -> Self {
        Packet { 
            head_len : HEAD_BYTES_SIZE,
            body_len : 0,
            opcode : 0,
            head_param : 0,
            head_uuid:0,
            head_time_stamp : 0,
            head_magic : PACKET_MAGIC_D,
            head_offset:0,
            head_buf : vec![],
            buf: vec![]
        }
    }
    // ========================== packetHead
    pub fn set_params(&mut self , key:PacketHeaderKey, data:usize){
        match key {
          PacketHeaderKey::OPCODE=> {
              self.opcode = u8::from_usize(data).unwrap();
          },
          PacketHeaderKey::BLEN=>{
              self.body_len= u8::from_usize(data).unwrap();
          },
          PacketHeaderKey::MAGIC=>{
              self.head_magic= u16::from_usize(data).unwrap();
          },
          PacketHeaderKey::PARAM=>{
              self.head_param= u8::from_usize(data).unwrap();
          },
          PacketHeaderKey::TIMESTAMP=>{
              self.head_time_stamp= u32::from_usize(data).unwrap();
          }
        }
     }
  
     pub fn get_params(&self,key:PacketHeaderKey)->usize{
      match key {
          PacketHeaderKey::OPCODE=> {
              usize::from_u8(self.opcode).unwrap()
          },
          PacketHeaderKey::BLEN=>{
              usize::from_u8(self.body_len).unwrap()
          },
          PacketHeaderKey::MAGIC=>{
              usize::from_u16(self.head_magic).unwrap()
          },
          PacketHeaderKey::PARAM=>{
              usize::from_u8(self.head_param).unwrap()
          },
          PacketHeaderKey::TIMESTAMP=>{
              usize::from_u32(self.head_time_stamp).unwrap()
          }
        }
     }
  
     pub fn get_buf(&mut self) -> &Vec<u8> {
         &self.buf
     }
  
     pub fn set_buf<'a>(&'a mut self ,buffer:&Vec<u8>){
        //  self.buf.clear();
         self.buf = buffer.clone()[..].to_vec();
     }
  
     pub fn update_offset<'a>(&'a mut self,offset:u8)-> &'a u8{
         self.head_offset += offset;
         &self.head_offset
     }
  
     pub fn pack<'a>(&'a mut self)-> &'a Vec<u8> {
         self.clean_buf();
         let mut vec_u8 = Vec::with_capacity(HEAD_BYTES_SIZE);
         let true_cap = vec_u8.capacity();
         unsafe {
             vec_u8.set_len(true_cap);
         };
         let mut output= vec_u8.into_boxed_slice();
         byteorder::NativeEndian::write_u16(&mut output[..2],self.head_magic);
         self.head_offset+=2;
         byteorder::NativeEndian::write_u32(&mut output[2..6],u32::from_u8(self.body_len).unwrap());
         self.head_offset+=4;
         byteorder::NativeEndian::write_u32(&mut output[6..10],u32::from_u8(self.opcode).unwrap());
         self.head_offset+=4;
         byteorder::NativeEndian::write_u32(&mut output[10..14],u32::from_u8(self.head_uuid).unwrap());
         self.head_offset+=4;
         byteorder::NativeEndian::write_u32(&mut output[14..18],u32::from_u8(self.head_param).unwrap());
         self.head_offset+=4;
         byteorder::NativeEndian::write_u64(&mut output[18..],u64::from_u32(self.head_time_stamp).unwrap());
         self.head_offset+=8;
         self.buf = output.as_ref().to_vec();
         &self.buf
     }
  
     pub fn un_pack(& mut self,buffer:&mut [u8]) -> u8 {
        let true_cap = buffer.len();
        if true_cap < HEAD_BYTES_SIZE {
            log!("Packet header length invalid.{:?}","error");
            return 0;
        }
        let mut offset:u8 = 0;
        self.head_magic = byteorder::NativeEndian::read_u16(&mut buffer[..2]);
        offset += 2;
  
        self.body_len = u8::from_u32(byteorder::NativeEndian::read_u32(&mut buffer[2..6])).unwrap();
        offset += 4;
    
        self.opcode =  u8::from_u32(byteorder::NativeEndian::read_u32(&mut buffer[6..10])).unwrap();
        offset += 4;
    
        // Do not read uuid
        self.head_uuid = u8::from_u32(byteorder::NativeEndian::read_u32(&mut buffer[10..14])).unwrap();
        offset += 4;
    
        self.head_param = u8::from_u32(byteorder::NativeEndian::read_u32(&mut buffer[14..18])).unwrap();
        offset += 4;
    
        self.head_time_stamp = byteorder::NativeEndian::read_u32(&mut buffer[18..]);
        offset += 8;
    
        offset
     }
  
     pub fn clean_buf(&mut self){
        self.head_offset = 0;
        self.head_buf.clear();
     }
    // ========================== packet
    pub fn serialization(&mut self) -> Vec<u8>{
        let buf:Vec<u8> ;
        self.mkHead();
        let head_buf = self.get_buf();
        let mut head_buf_available = head_buf.clone();
        let len = cmp::min(head_buf_available[HEAD_BYTES_SIZE..].len(), self.buf.len());
        head_buf_available[HEAD_BYTES_SIZE..][..len].copy_from_slice(&self.buf[..len]);

        self.head_offset += 1;
        buf = self.get_buf().to_vec();
        // buf.split_at(HEAD_BYTES_SIZE + len);
        buf
    }

    pub fn deserialization(&mut self,buf:&mut [u8]) -> bool{
        if buf.len() < HEAD_BYTES_SIZE {
            log!("Packet is smaller than header size.{:?}","error");
            return false;
        }
      
        let pos = usize::from_u8(self.un_pack(buf)).unwrap();
        let content_len = self.get_params(PacketHeaderKey::BLEN);
        unsafe {
            self.buf.set_len(content_len);
        };
        self.buf = buf[0..pos].to_vec();
        false
    }

    pub fn peek_body_length(&mut self , buf:&mut [u8]) -> usize{
        let mut body_len = usize::from_i8(-1).unwrap();
        if buf.len() < HEAD_BYTES_SIZE {
            return body_len
        }
        
        self.un_pack(buf);
        body_len = self.get_params(PacketHeaderKey::BLEN);
        body_len
    }

    pub fn init(&mut self, opcode: usize, param:usize) {
        self.set_params(PacketHeaderKey::OPCODE, opcode);
        self.set_params(PacketHeaderKey::PARAM, param);
    }

    fn mkHead(&mut self){
        self.set_params(PacketHeaderKey::BLEN, self.buf.len());
        let head_buf = self.pack();
        self.buf = head_buf.clone()[..].to_vec();
    }
}

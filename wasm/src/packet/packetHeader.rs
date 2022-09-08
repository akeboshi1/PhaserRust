
extern crate buf_redux;
extern crate byteorder; use std::u8;

// 1.2.7
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use byteorder::{ByteOrder};

use crate::log;

const S : u16 = 83;// utils::stringutil::get_char_code(&"S"); 83
const D : u16 = 68;// utils::stringutil::get_char_code(&"D"); 68
const C : u16 = 67;// utils::stringutil::get_char_code(&"C"); 67
const PACKET_MAGIC_D:u16 = (S << 8) | D; //'DS'   default=not zip
const PACKET_MAGIC_C:u16 = (S << 8) | C; //'CS'    zip
pub const HEAD_BYTES_SIZE:u32 = 2 + 4 + 4 + 4 + 4 + 8;


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
   * |   timestamp   +  double  +  8Bytes  +  parameter of packet |
   * +------------------------------------------------------------+
   */
pub struct PacketHeader {
    head_len :u32,
    len:u32,
    opcode:u32,
    param:u32,
    uuid:u32,
    time_stamp:f64,
    magic:u16,
    offset:u32,
    buf:Vec<u8>
}


impl PacketHeader{
   pub fn new()-> Self {
      PacketHeader {
        head_len : HEAD_BYTES_SIZE,
        len : 0,
        opcode : 0,
        param : 0,
        uuid:0,
        time_stamp : 0.0,
        magic : PACKET_MAGIC_D,
        offset:0,
        buf : vec![]
        // write:BufWriter::with_capacity(0, inner),
        // read:BufReader::new(R)
      }
   }
   // ================== 静态数据
   pub fn set_magic(&mut self,data:u16)->u16{
      self.magic = data;
      self.magic
   }

   pub fn get_magic(&mut self)->u16{
      self.magic
   }

   pub fn set_opcode(&mut self,data:u32)->u32{
      self.opcode = data;
      self.opcode
   }

   pub fn get_opcode(&mut self)->u32{
      self.opcode
   }

   pub fn set_timestamp(&mut self,data:f64)->f64{
     self.time_stamp = data;
     self.time_stamp
   }

   pub fn get_timestamp(&mut self)->f64{
     self.time_stamp
   }

   // ================== 动态数据
   pub fn set_blen(&mut self,data:u32)->u32{
      self.len = data;
      self.len
   }

   pub fn get_blen(&self)->u32{
      self.len
   }

   pub fn set_param(&mut self,data:u32)->u32{
      self.param = data;
      self.param
   }

   pub fn get_param(&self)->u32{
      self.param
   }

   pub fn set_buf(&mut self ,buffer:Vec<u8>){
       self.buf.clear();
       self.buf = buffer;
   }

   pub fn get_buf(&self) -> Vec<u8> {
       self.buf
   }

   pub fn update_offset(&mut self,offset:u32)-> u32{
       self.offset += offset;
       self.offset
   }

   pub fn pack(&mut self)-> Vec<u8> {
       self.clean_buf();
       let mut vec_u8 = Vec::with_capacity(u8::from_u32(HEAD_BYTES_SIZE).unwrap().into());
       let true_cap = vec_u8.capacity();
       unsafe {
           vec_u8.set_len(true_cap);
       };
       let output= vec_u8.into_boxed_slice();
       byteorder::NativeEndian::write_u16(&mut output[..2],self.magic);
       self.offset+=2;
       byteorder::NativeEndian::write_u32(&mut output[2..6],self.len);
       self.offset+=4;
       byteorder::NativeEndian::write_u32(&mut output[6..10],self.opcode);
       self.offset+=4;
       byteorder::NativeEndian::write_u32(&mut output[10..14],self.uuid);
       self.offset+=4;
       byteorder::NativeEndian::write_u32(&mut output[14..18],self.param);
       self.offset+=4;
       byteorder::NativeEndian::write_f64(&mut output[18..],self.time_stamp);
       self.offset+=8;
       self.buf = output.as_ref().to_vec();
       self.buf
   }

   pub fn un_pack(&mut self,buffer:&[u8]) -> usize {
      let true_cap = buffer.len();
      if true_cap < u8::from_u32(HEAD_BYTES_SIZE).unwrap().into() {
          log!("Packet header length invalid.{:?}","error");
          return 0;
      }
      let offset = 0;
      self.magic = byteorder::NativeEndian::read_u16(&mut buffer[..2]);
      offset += 2;

      self.len = byteorder::NativeEndian::read_u32(&mut buffer[2..6]);
      offset += 4;
  
      self.opcode =  byteorder::NativeEndian::read_u32(&mut buffer[6..10]);
      offset += 4;
  
      // Do not read uuid
      self.uuid = byteorder::NativeEndian::read_u32(&mut buffer[10..14]);
      offset += 4;
  
      self.param = byteorder::NativeEndian::read_u32(&mut buffer[14..18]);
      offset += 4;
  
      self.time_stamp = byteorder::NativeEndian::read_f64(&mut buffer[18..]);
      offset += 8;
  
      offset
   }

   pub fn clean_buf(&mut self){
      self.offset = 0;
      self.buf.clear();
   }
}

extern crate buf_redux;
extern crate byteorder; // 1.2.7
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use byteorder::{ByteOrder, WriteBytesExt};
use buf_redux::{BufWriter, BufReader, Buffer};
use std::{io::{Write,Read}, fmt::write};

use crate::log;

const S : u16 = 83;// utils::stringutil::get_char_code(&"S"); 83
const D : u16 = 68;// utils::stringutil::get_char_code(&"D"); 68
// const C : u8 = 67;// utils::stringutil::get_char_code(&"C"); 67
const PACKET_MAGIC_D:u16 = (S << 8) | D; //'DS'   default=not zip
// const PACKET_MAGIC_C:u8 = (S << 8) | C; //'CS'    zip
const HEAD_BYTES_SIZE:usize = 2 + 4 + 4 + 4 + 4 + 8;


#[repr(u8)]
#[derive(FromPrimitive)]
enum PacketHeaderKey {
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
pub struct PacketHeader {
    head_len :usize,
    len:u8,
    opcode:u8,
    param:u8,
    uuid:u8,
    time_stamp:u32,
    magic:u16,
}

impl PacketHeader{
   pub fn new()-> Self {
      PacketHeader {
        head_len : HEAD_BYTES_SIZE,
        len : 0,
        opcode : 0,
        param : 0,
        uuid:0,
        time_stamp : 0,
        magic : PACKET_MAGIC_D,
        // write:BufWriter::with_capacity(0, inner),
        // read:BufReader::new(R)
      }
   }

   pub fn set_params(&mut self , key:u8, data:usize){
      match FromPrimitive::from_u8(key) {
        Some(PacketHeaderKey::OPCODE)=> {
            self.opcode = u8::from_usize(data).unwrap();
        },
        Some(PacketHeaderKey::BLEN)=>{
            self.len= u8::from_usize(data).unwrap();
        },
        Some(PacketHeaderKey::MAGIC)=>{
            self.magic= u16::from_usize(data).unwrap();
        },
        Some(PacketHeaderKey::PARAM)=>{
            self.param= u8::from_usize(data).unwrap();
        },
        Some(PacketHeaderKey::TIMESTAMP)=>{
            self.time_stamp= u32::from_usize(data).unwrap();
        },
        _=>println!("packetHeader default set"),
      }
   }

   pub fn get_params(&self,key:u8)->usize{
    match FromPrimitive::from_u8(key) {
        Some(PacketHeaderKey::OPCODE)=> {
            usize::from_u8(self.opcode).unwrap()
        },
        Some(PacketHeaderKey::BLEN)=>{
            usize::from_u8(self.len).unwrap()
        },
        Some(PacketHeaderKey::MAGIC)=>{
            usize::from_u16(self.magic).unwrap()
        },
        Some(PacketHeaderKey::PARAM)=>{
            usize::from_u8(self.param).unwrap()
        },
        Some(PacketHeaderKey::TIMESTAMP)=>{
            usize::from_u32(self.time_stamp).unwrap()
        }
        None => todo!(),
      }
   }

   pub fn pack(&mut self)-> &[u8] {
       self.clean_buf();
       let mut vec_u8 = Vec::with_capacity(HEAD_BYTES_SIZE);
       let true_cap = vec_u8.capacity();
       unsafe {
           vec_u8.set_len(true_cap);
       };
       let output= vec_u8.into_boxed_slice();
       byteorder::NativeEndian::write_u16(&mut output[..2],self.magic);
       byteorder::NativeEndian::write_u32(&mut output[2..6],u32::from_u8(self.len).unwrap());
       byteorder::NativeEndian::write_u32(&mut output[6..10],u32::from_u8(self.opcode).unwrap());
       byteorder::NativeEndian::write_u32(&mut output[10..14],u32::from_u8(self.uuid).unwrap());
       byteorder::NativeEndian::write_u32(&mut output[14..18],u32::from_u8(self.param).unwrap());
       byteorder::NativeEndian::write_u64(&mut output[18..],u64::from_u32(self.time_stamp).unwrap());

       output.as_ref()
   }

   pub fn un_pack(&mut self,buffer:&[u8]) -> usize {
      let true_cap = buffer.len();
      if true_cap < HEAD_BYTES_SIZE {
          log!("Packet header length invalid.{:?}","error");
          return 0;
      }
      let offset = 0;
      self.magic = byteorder::NativeEndian::read_u16(&mut buffer[..2]);

      offset += 2;

      self.len = u8::from_u32(byteorder::NativeEndian::read_u32(&mut buffer[2..6])).unwrap();
      offset += 4;
  
      self.opcode =  u8::from_u32(byteorder::NativeEndian::read_u32(&mut buffer[6..10])).unwrap();
      offset += 4;
  
      // Do not read uuid
      self.uuid = u8::from_u32(byteorder::NativeEndian::read_u32(&mut buffer[10..14])).unwrap();
      offset += 4;
  
      self.param = u8::from_u32(byteorder::NativeEndian::read_u32(&mut buffer[14..18])).unwrap();
      offset += 4;
  
      self.time_stamp = byteorder::NativeEndian::read_u32(&mut buffer[18..]);
      offset += 8;
  
      offset
   }

   pub fn clean_buf(&self){
   }
}
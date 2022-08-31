extern crate byteorder; // 1.2.7
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use byteorder::ByteOrder;

use crate::buffer::buffer::Buffer;
const S : u16 = 83;// utils::stringutil::get_char_code(&"S"); 83
const D : u16 = 68;// utils::stringutil::get_char_code(&"D"); 68
// const C : u8 = 67;// utils::stringutil::get_char_code(&"C"); 67
const PACKET_MAGIC_D:u16 = (S << 8) | D; //'DS'   default=not zip
// const PACKET_MAGIC_C:u8 = (S << 8) | C; //'CS'    zip
const HEAD_BYTES_SIZE:u8 = 2 + 4 + 4 + 4 + 4 + 8;


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
    head_len :u8,
    len:u8,
    offset:u8,
    opcode:u8,
    param:u8,
    uuid:u8,
    time_stamp:u32,
    magic:u16,
    buf: Buffer
}

impl PacketHeader{
   pub fn new()-> Self {
      PacketHeader {
        head_len : HEAD_BYTES_SIZE,
        len : 0,
        offset : 0,
        opcode : 0,
        param : 0,
        uuid:0,
        time_stamp : 0,
        magic : PACKET_MAGIC_D,
        buf: Buffer::new(0)
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

   pub fn pack(&mut self)-> &Buffer {
       self.clean_buf();
       byteorder::NativeEndian::write_u16(&mut self.buf.bytes_mut(),self.magic);
       self.offset += 2;
       byteorder::NativeEndian::write_u32(&mut self.buf.bytes_mut(), u32::from_u8(self.len).unwrap());
       self.offset +=4;
       byteorder::NativeEndian::write_u32(&mut self.buf.bytes_mut(),u32::from_u8(self.opcode).unwrap());
       self.offset +=4;
       byteorder::NativeEndian::write_u32(&mut self.buf.bytes_mut(), u32::from_u8(self.uuid).unwrap());
       self.offset +=4;
       byteorder::NativeEndian::write_u32(&mut self.buf.bytes_mut(),u32::from_u8(self.param).unwrap());
       self.offset +=4;
       byteorder::NativeEndian::write_u64(&mut self.buf.bytes_mut(), u64::from_u32(self.time_stamp).unwrap());
       self.offset +=8;
       &self.buf
   }


   pub fn clean_buf(&mut self){
      self.buf.clear();
      self.offset = 0;
   }

}
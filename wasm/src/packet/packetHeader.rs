extern crate buffer;
use buffer::BufferRef;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

const S : u32 = 83;// utils::stringutil::get_char_code(&"S"); 83
const D : u32 = 68;// utils::stringutil::get_char_code(&"D"); 68
const C : u32 = 67;// utils::stringutil::get_char_code(&"C"); 67
const PACKET_MAGIC_D:u32 = (S << 8) | D; //'DS'   default=not zip
const PACKET_MAGIC_C:u32 = (S << 8) | C; //'CS'    zip
const HEAD_BYTES_SIZE:u32 = 2 + 4 + 4 + 4 + 4 + 8;


#[repr(u16)]
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
    head_len :u32,
    len:u32,
    offset:u32,
    opcode:u32,
    param:u32,
    time_stamp:u32,
    magic:u32,
}

impl<'a,'b> PacketHeader{
   pub fn new()-> Self {
      PacketHeader {
        head_len : HEAD_BYTES_SIZE,
        len : 0,
        offset : 0,
        opcode : 0,
        param : 0,
        time_stamp : 0,
        magic : PACKET_MAGIC_D,
      }
   }

   pub fn init_buffer(&self,buffer: &'a mut [u8], initialized: &'b mut usize) -> BufferRef<'a, 'b> {
       BufferRef::new(buffer, initialized)
   }

   pub fn set_params(&mut self , key:u8, data:u32){
      match FromPrimitive::from_u8(key) {
        Some(PacketHeaderKey::OPCODE)=> {
            self.opcode = data;
        },
        Some(PacketHeaderKey::BLEN)=>{
            self.len= data;
        },
        Some(PacketHeaderKey::MAGIC)=>{
            self.magic= data;
        },
        Some(PacketHeaderKey::PARAM)=>{
            self.param= data;
        },
        Some(PacketHeaderKey::TIMESTAMP)=>{
            self.time_stamp= data;
        },
        _=>println!("packetHeader default set"),
      }
   }

   pub fn get_params(&self,key:u8)->u32{
    match FromPrimitive::from_u8(key) {
        Some(PacketHeaderKey::OPCODE)=> {
            self.opcode
        },
        Some(PacketHeaderKey::BLEN)=>{
            self.len
        },
        Some(PacketHeaderKey::MAGIC)=>{
            self.magic.into()
        },
        Some(PacketHeaderKey::PARAM)=>{
            self.param
        },
        Some(PacketHeaderKey::TIMESTAMP)=>{
            self.time_stamp
        }
        None => todo!(),
      }
   }

   pub fn pack(&self,buffer:BufferRef)-> BufferRef {
       &self.clean_buf(buffer);
      
       buffer
   }

   pub fn clean_buf(&mut self,buffer:BufferRef){
     buffer::
      self.offset = 0;
   }


}
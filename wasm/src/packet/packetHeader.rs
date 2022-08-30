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
pub struct PacketHeader<'a,'b> {
    head_len :u32,
    len:u32,
    offset:u32,
    opcode:u32,
    param:u32,
    time_stamp:u32,
    magic:u32,
    buf:BufferRef<'a, 'b>
}

impl <'a,'b>PacketHeader<'a, 'b>{
   pub fn new(buffer: &'a mut [u8], initialized: &'b mut usize)-> Self {
      PacketHeader {
        head_len : HEAD_BYTES_SIZE,
        len : 0,
        offset : 0,
        opcode : 0,
        param : 0,
        time_stamp : 0,
        magic : PACKET_MAGIC_D,
        buf: BufferRef::new(buffer, initialized)
      }
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

   pub fn pack(&mut self,buffer: &'a mut [u8], initialized: &'b mut usize)-> &BufferRef<'a,'b> {
       self.clean_buf(buffer,initialized);

    //    .extend(bytes.iter().cloned())
    //    self.buf.write(self.magic)
    //    this._buf.writeUInt16LE(this._magic, this._offset);
    // this._offset += 2;

    // this._buf.writeUInt32LE(this._len, this._offset);
    // this._offset += 4;

    // this._buf.writeUInt32LE(this._opcode, this._offset);
    // this._offset += 4;

    // this._buf.writeUInt32LE(this._uuid, this._offset);
    // this._offset += 4;

    // this._buf.writeUInt32LE(this._param, this._offset);
    // this._offset += 4;

    // this._buf.writeDoubleLE(this._timestamp, this._offset);
    // this._offset += 8;
       &self.buf
   }

   pub fn clean_buf(&mut self,buffer:&'a mut [u8], initialized: &'b mut usize){
      self.buf = BufferRef::new(buffer, initialized);
      self.offset = 0;
   }


}
use js_sys::ArrayBuffer;   
use num_traits::FromPrimitive;
use crate::log;

use super::packet::HEAD_BYTES_SIZE;

pub struct PBPacket{

}

trait PBTrait {
    fn new()-> Self;
    fn create(data:ArrayBuffer)-> PBPacket;
    fn set_up_from_arraybuffer(&self,data:ArrayBuffer);
}

impl PBTrait for PBPacket{
    fn new()-> Self {
        PBPacket {
        }
    }

    fn set_up_from_arraybuffer(&self,data:ArrayBuffer) {
        let byte_len = usize::from_u32(data.byte_length()).unwrap();
        if byte_len < HEAD_BYTES_SIZE {
            log!("Packet is smaller than header size. {:?}","error");
            return;
        }
        
    }

    fn create(data:ArrayBuffer)-> PBPacket {
        let packet = PBPacket::new();
        packet.set_up_from_arraybuffer(data);
        packet
    }
}
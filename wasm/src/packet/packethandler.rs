use super::packet::Packet;
pub trait PacketListener {
    fn onPacketArrived(packet:Packet)->bool;
    fn addHandlerFun(opcode:u8,callback:js_sys::Function);
}

pub struct PacketHandler{

}

impl PacketListener for PacketHandler{
    fn onPacketArrived(packet: Packet)->bool{
        false
    }

    fn addHandlerFun(opcode: u8,callback:js_sys::Function){
        
    }
}
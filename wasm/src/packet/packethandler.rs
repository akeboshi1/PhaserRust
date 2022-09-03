use super::packet::Packet;
trait PacketListener {
    fn onPacketArrived(packet:Packet)->bool;
}
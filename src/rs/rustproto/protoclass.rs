use std::any::Any;
use serde::{Serialize, Deserialize};
pub trait AsAny: Any {
    fn as_any(&self)-> &dyn Any;
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ProtoClass{
    // opcode:u8,
}


impl AsAny for ProtoClass {
     fn as_any(&self) -> &dyn Any {
        self
     }
}

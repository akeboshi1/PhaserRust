use std::any::Any;
use js_sys::Object;
use serde::{Serialize, Deserialize};
pub trait AsAny: Any {
    fn as_any(&self)-> &dyn Any;
}

#[derive(Debug)]
pub struct ProtoClass{
    obj:Object
}


impl AsAny for ProtoClass {
     fn as_any(&self) -> &dyn Any {
        self
     }
}

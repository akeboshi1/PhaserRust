use std::any::Any;
pub trait AsAny: Any {
    fn as_any(&self)-> &dyn Any;
}

pub struct ProtoClass{
    
}


impl AsAny for ProtoClass {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

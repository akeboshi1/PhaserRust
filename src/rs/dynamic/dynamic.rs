use std::any::Any;

use js_sys::Function;

pub struct Dynamic {
    type_name: Function,
    data: Box<dyn Any>,
}

impl Dynamic{
    pub fn new<T>(value: T) -> Self
    where
        T: 'static,
       {
          Dynamic {
            type_name: rust_type::<T>(),
            data: Box::new(value),
          }
       }
}



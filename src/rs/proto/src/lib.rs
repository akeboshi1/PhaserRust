mod rust_protobuf_protos {
    include!(concat!(env!("OUT_DIR"), "/rust_protobuf_protos/mod.rs"));
}

mod prost_protos {
    include!(concat!(env!("OUT_DIR"), "/op_def.rs"));
}

#[cfg(test)]
mod test {
    use crate::prost_protos::*;
    use crate::rust_protobuf_protos::*;

    #[test]
    fn main() {
        let mut rp = rust_protobuf_protos::triangle::ConnectRequest::new();

        rp.server_name = "Test servename";

    }
}
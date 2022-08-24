
use std::io::Result;
fn main() -> Result<()> {

    protobuf_codegen::Codegen::new()
        .include("src")
        .inputs(["src/test.proto"])
        .cargo_out_dir("rust_protobuf_protos")
        .run_from_script();

    prost_build::Config::new()
        .compile_protos(&["src/test.proto"], &["src"])?;
        Ok(())
}


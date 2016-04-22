extern crate protobuf_build;

use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source = root.join("proto");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut compiler = protobuf_build::Compiler::new(&source, &out);
    compiler.compile("fileformat.proto").unwrap();
    compiler.compile("osmformat.proto").unwrap();
}

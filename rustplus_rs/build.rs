use std::path::Path;

fn main() {
    let proto_file = "proto/rustplus.proto";
    
    if Path::new(proto_file).exists() {
        println!("cargo:rerun-if-changed={}", proto_file);
        
        // Try to compile protobuf, but don't fail if protoc is not available
        match prost_build::compile_protos(&[proto_file], &["proto/"]) {
            Ok(_) => println!("cargo:warning=Protobuf compilation successful"),
            Err(e) => {
                println!("cargo:warning=Protobuf compilation failed: {}", e);
                println!("cargo:warning=Using manual protobuf definitions instead");
            }
        }
    } else {
        println!("cargo:warning=Proto file not found: {}", proto_file);
    }
}

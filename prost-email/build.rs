fn main() {
    prost_build::compile_protos(&["proto/email.proto"], &["proto/"]).unwrap();
}

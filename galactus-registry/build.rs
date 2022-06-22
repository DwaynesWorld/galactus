fn main() {
    tonic_build::compile_protos("proto/config.proto").unwrap();
}

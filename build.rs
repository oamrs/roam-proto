fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    tonic_build::configure()
        .compile(
            &["src/v1/agent/service.proto"],
            &["src"],
        )?;
    Ok(())
}

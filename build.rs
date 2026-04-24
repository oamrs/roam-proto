fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    tonic_build::configure().compile(
        &[
            "src/v1/agent/service.proto",
            "src/v1/control/service.proto",
            "src/v1/query/service.proto",
            "src/v1/schema/service.proto",
        ],
        &["src"],
    )?;
    Ok(())
}

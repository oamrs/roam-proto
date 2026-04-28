fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prefer an externally-set PROTOC; fall back to the bundled binary only
    // when it actually exists at the path protobuf-src compiled it to.
    if std::env::var("PROTOC").is_err() {
        let bundled = protobuf_src::protoc();
        if bundled.exists() {
            std::env::set_var("PROTOC", &bundled);
        }
        // If neither is set and the bundled path is stale (e.g. after a workspace
        // rename), tonic_build will look for `protoc` on PATH as a last resort.
    }
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

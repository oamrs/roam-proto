# oam-proto

Shared Protocol Buffer definitions for the [OAM (Object Agent Mapping)](https://github.com/oamrs/roam) framework. This crate is the contract between the OAM backend and all client SDKs (Rust, Python, .NET, Go).

## Services

### `AgentService` — Agent registration and event streaming

```protobuf
service AgentService {
  rpc Register (ConnectRequest) returns (ConnectResponse);
  rpc StreamEvents (EventStreamRequest) returns (stream Event);
}
```

Agents connect with a `SchemaMode` that controls data access:

| Mode | Description |
|------|-------------|
| `DATA_FIRST` | DB introspection — read-only access to legacy data |
| `CODE_FIRST` | Registered models only — read-write with code-defined validation |
| `HYBRID` | Registered models + introspection fallback — read-only |

### `QueryService` — SQL query execution

```protobuf
service QueryService {
  rpc ExecuteQuery (ExecuteQueryRequest) returns (ExecuteQueryResponse);
  rpc ValidateQuery (ValidateQueryRequest) returns (ValidationResponse);
}
```

### `SchemaService` — Database schema introspection

```protobuf
service SchemaService {
  rpc GetSchema (GetSchemaRequest) returns (GetSchemaResponse);
  rpc GetTable  (GetTableRequest)  returns (GetTableResponse);
}
```

## Usage in Rust

```toml
[dependencies]
oam-proto = "0.1"
tonic = "0.11"
prost = "0.12"
```

```rust
use oam_proto::v1::agent::agent_service_client::AgentServiceClient;
use oam_proto::v1::query::query_service_client::QueryServiceClient;

let mut client = AgentServiceClient::connect("http://[::1]:50051").await?;
```

Stubs for other languages are generated from the same `.proto` sources via `grpcio-tools` (Python) or `Grpc.Tools` (.NET).

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.

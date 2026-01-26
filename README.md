# ROAM Protocol Definitions

This library contains the shared Protocol Buffer definitions for the ROAM framework.
It serves as the contract between the Backend and all client SDKs.

## Structure

- `src/v1/agent/`: Agent communication protocols (gRPC).

## Usage

This project does not contain application code. It is used to generate client/server stubs for:
- Rust (via `tonic-build`)
- Python (via `grpcio-tools`)
- .NET (via `Grpc.Tools`)

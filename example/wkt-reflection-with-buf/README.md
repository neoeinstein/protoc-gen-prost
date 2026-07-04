# Well-known types with buf and gRPC reflection

This example shows how to wire [tonic-reflection] when you generate Rust
protobuf code with [buf] and `protoc-gen-prost`, map well-known types to
`pbjson-types`, and serve a gRPC API that references types such as
`google.protobuf.Timestamp`.

[tonic-reflection]: https://docs.rs/tonic-reflection
[buf]: https://buf.build

## Problem

With the buf workflow, each generated module exposes its own
`FILE_DESCRIPTOR_SET` constant (see `file_descriptor_set` in
[`protoc-gen-prost` options](../../protoc-gen-prost/README.md)). Well-known
types are usually redirected with:

```yaml
opt:
  - compile_well_known_types
  - extern_path=.google.protobuf=::pbjson_types
```

Those types are **not** included in your service descriptor set. For gRPC
reflection to describe messages that embed `google.protobuf.Timestamp` (or
other well-known types), you must also register the descriptor set shipped by
the [`protoc-wkt`](../../protoc-wkt) crate.

## Layout

| Path | Role |
| --- | --- |
| `proto/example/*.proto` | Sample service using `google.protobuf.Timestamp` |
| `buf.gen.yaml` | buf plugins and generation options |
| `.build.rs` | Invokes `buf generate` from `build.rs` |
| `src/lib.rs` | Registers generated + WKT descriptor sets for reflection |

## Generate and build

Build the workspace plugins first (from the repository root):

```shell
cargo build --release --locked
export PATH="$PWD/target/release:$PATH"
```

Then generate and compile this example:

```shell
cd example/wkt-reflection-with-buf
buf generate
cargo build --locked
cargo test --locked
```

CI runs the same steps in the `wkt-reflection-with-buf` job (see
[`.github/workflows/rust.yml`](../../.github/workflows/rust.yml)).

## Reflection registration

The critical piece is registering **both** descriptor sets:

```rust
use protoc_wkt::google::protobuf::FILE_DESCRIPTOR_SET as GOOGLE_PROTOBUF_FILE_DESCRIPTOR_SET;

tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(example::FILE_DESCRIPTOR_SET)
    .register_encoded_file_descriptor_set(GOOGLE_PROTOBUF_FILE_DESCRIPTOR_SET)
    .build_v1()?;
```

Register a descriptor set for **every** protobuf file your API depends on,
recursively. Missing imports surface as reflection errors at runtime even when
the server binary compiles cleanly.

## Related

- [`protoc-wkt` README](../../protoc-wkt/README.md)
- [`example/build-with-buf`](../build-with-buf) — buf + `build.rs` without reflection
- Issue [#64](https://github.com/neoeinstein/protoc-gen-prost/issues/64)

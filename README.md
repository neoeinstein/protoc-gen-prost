# protoc-gen-prost

A `protoc` plugin that generates code using the _[Prost!]_ code generation engine.

[Prost!]: https://github.com/tokio-rs/prost

When used in projects that use only Rust code, the preferred mechanism for
generating protobuf definitions with _Prost!_ is to use [`prost-build`] from
within a `build.rs` file. However, when working in polyglot environments,
it can be advantageous to utilize common tooling in the Protocol Buffers
ecosystem. One common tool used for this purpose is _[buf]_, which simplifies
the code generation process and includes several useful features, including
linting, package management, and breaking change detection.

[`prost-build`]: https://docs.rs/prost-build
[buf]: https://buf.build

## Usage

The various modules that are used for generating Rust code with _Prost!_ and
_Tonic_ are available in the named subdirectories. Refer to the README in
each of those folders for more information.

* _[protoc-gen-prost]_: The core code generation plugin
* _[protoc-gen-prost-crate]_: Generates an include file and cargo manifest for turn-key crates
* _[protoc-gen-prost-serde]_: Canonical JSON serialization of protobuf types
* _[protoc-gen-prost-utoipa]_: _[utoipa]_ schemas of protobuf types
* _[protoc-gen-prost-validate]_: Generate validators based on embedded metadata
* _[protoc-gen-tonic]_: gRPC service generation for the _[Tonic]_ framework

[protoc-gen-prost]: protoc-gen-prost/README.md
[protoc-gen-prost-crate]: protoc-gen-prost-crate/README.md
[protoc-gen-prost-serde]: protoc-gen-prost-serde/README.md
[protoc-gen-prost-utoipa]: protoc-gen-prost-utoipa/README.md
[utoipa]: https://github.com/juhaku/utoipa
[protoc-gen-prost-validate]: protoc-gen-prost-validate/README.md
[protoc-gen-tonic]: protoc-gen-tonic/README.md
[Tonic]: https://github.com/hyperium/tonic

## Example `buf.gen.yaml`

Note: When executing `protoc-gen-prost-crate` with the `gen_crate` option, remote
generation is not possible, as the manifest template is not made available to a
remote plugin.

```yaml
version: v1
plugins:
  - remote: buf.build/prost/plugins/prost:v0.2.1-1
    out: gen/src
    opt:
      - bytes=.
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
      - file_descriptor_set
  - remote: buf.build/prost/plugins/serde:v0.2.1-1
    out: gen/src
  - remote: buf.build/prost/plugins/tonic:v0.2.1-1
    out: gen/src
    opt:
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
  - name: prost-crate
    out: gen
    strategy: all
    opt:
      - gen_crate=Cargo.toml
```

See `example/build-with-buf` for an example of invoking `buf` as part of a
`build.rs` build script. This may be useful when you want to extend the
generated files with additional trait or inherent implementations.

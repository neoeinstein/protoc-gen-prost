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
* _[protoc-gen-prost-validate]_: Generate validators based on embedded metadata
* _[protoc-gen-tonic]_: Tonic gRPC service generation

[protoc-gen-prost]: protoc-gen-prost/README.md
[protoc-gen-prost-crate]: protoc-gen-prost-crate/README.md
[protoc-gen-prost-serde]: protoc-gen-prost-serde/README.md
[protoc-gen-prost-validate]: protoc-gen-prost-validate/README.md
[protoc-gen-tonic]: protoc-gen-tonic/README.md

## Example `buf.gen.yaml`

```yaml
version: v1
plugins:
  - name: prost-crate
    out: gen
    strategy: all
    opt:
      - gen_crate=Cargo.toml.tpl
  - name: prost
    out: gen/src
    opt:
      - bytes=.
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
      - file_descriptor_set
  - name: prost-serde
    out: gen/src
    opt:
      - extern_path=.google.protobuf=::pbjson_types
  - name: tonic
    out: gen/src
```

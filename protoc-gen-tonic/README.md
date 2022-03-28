# protoc-gen-tonic

A `protoc` plugin that generates _[Tonic]_ gRPC server and client code using
the _[Prost!]_ code generation engine.

[Tonic]: https://github.com/hyperium/tonic
[Prost!]: https://github.com/tokio-rs/prost

When used in projects that use only Rust code, the preferred mechanism for
generating gRPC services with _Tonic_ is to use [`tonic-build`] from
within a `build.rs` file. However, when working in polyglot environments,
it can be advantageous to utilize common tooling in the Protocol Buffers
ecosystem. One common tool used for this purpose is _[buf]_, which simplifies
the code generation process and includes several useful features, including
linting, package management, and breaking change detection.

[`tonic-build`]: https://docs.rs/tonic-build
[buf]: https://buf.build

## Usage

Ensure that `protoc-gen-tonic` has been installed within a directory on your
`$PATH`. Then invoke `protoc` from the command line as follows:

```shell
protoc --tonic_out=proto/gen -I proto proto/greeter/v1/greeter.proto
```

### Options

This tool supports all the same options from `tonic-build` except for those
that are expected to have been completely handled in an earlier
`protoc-gen-prost` step. For information on the effects of these settings,
see the related documentation from `tonic-build`:

    no_server: bool,
    no_client: bool,


* `default_package_filename=<value>`: [default_package_filename](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.default_package_filename)
* `extern_path=<proto_path>=<rust_path>`:  [extern_path](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.extern_path)
* `compile_well_known_types(=<boolean>)`: [compile_well_known_types](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.compile_well_known_types)
* `disable_package_emission(=<boolean>)`: [compile_well_known_types](https://docs.rs/tonic-build/latest/tonic_build/struct.Builder.html#method.disable_package_emission)
* `server_attribute=<proto_path>=<attribute>`: [server_attribute](https://docs.rs/tonic-build/latest/tonic_build/struct.Builder.html#method.server_attribute)
* `server_mod_attribute=<proto_path>=<attribute>`: [server_mod_attribute](https://docs.rs/tonic-build/latest/tonic_build/struct.Builder.html#method.server_mod_attribute)
* `client_attribute=<proto_path>=<attribute>`: [client_attribute](https://docs.rs/tonic-build/latest/tonic_build/struct.Builder.html#method.client_attribute)
* `client_mod_attribute=<proto_path>=<attribute>`: [client_mod_attribute](https://docs.rs/tonic-build/latest/tonic_build/struct.Builder.html#method.client_mod_attribute)

In addition, the following options can also be specified:

* `no_server(=<boolean>)`: Disables generation of the server modules
* `no_client(=<boolean>)`: Disables generation of the client modules

A note on parameter values:

* `<attribute>`: All `,`s appearing in the value must be `\` escaped
  (i.e. `\,`) This is due to the fact that internally, `protoc` joins all
  passed parameters with a `,` before sending it as a single string to the
  underlying plugin.
* `<proto_path>`: Protobuf paths beginning with `.` will be matched from the
  global root (prefix matches). All other paths will be matched as suffix
  matches.
* `(=<boolean>)`: Boolean values may be specified after a parameter, but if
  not, the value is assumed to be `true` by virtue of having listed the
  parameter.

### Usage with _buf_

When used with _buf_, options can be specified in the `buf.gen.yaml` file.
`protoc-gen-prost-tonic` should appear as a plugin step after any
`protoc-gen-prost` steps. In addition, 

```yaml
version: v1
plugins:
  - name: prost
    out: gen
  - name: tonic
    out: gen
```

If you have specified `compile_well_known_types` or `extern_path` on any
earlier step, those should also be specified for this step.

Pulling all of these together, you can compile a ready-made crate for a gRPC
service with types that can be JSON serialized using the following example.
Then you can depend on this crate from the binary that will host the server
or use the client.

```yaml
version: v1
plugins:
  - name: prost
    out: gen/src
    opt:
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
  - name: prost-serde
    out: gen/src
  - name: tonic
    out: gen/src
    opt:
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
  - name: prost
    out: gen
    opt:
      - gen_crate=Cargo.toml.tpl
```

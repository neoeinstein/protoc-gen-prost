# protoc-gen-prost-serde

A `protoc` plugin that generates _[serde]_ serialization implementations that
follow the conventions of protobuf-JSON.

[serde]: https://serde.rs

When used in projects that use only Rust code, the preferred mechanism for
generating protobuf definitions with _Prost!_ is to use [`prost-build`] from
within a `build.rs` file and then generate _serde_ implementations using
[`pbjson-build`]. However, when working in polyglot environments,
it can be advantageous to utilize common tooling in the Protocol Buffers
ecosystem. One common tool used for this purpose is _[buf]_, which simplifies
the code generation process and includes several useful features, including
linting, package management, and breaking change detection.

[`prost-build`]: https://docs.rs/prost-build
[`pbjson-build`]: https://docs.rs/pbjson-build
[buf]: https://buf.build

## Usage

Ensure that `protoc-gen-prost-serde` has been installed within a directory
on your `$PATH`. Then invoke `protoc` from the command line as follows:

```shell
protoc --prost-serde_out=proto/gen -I proto proto/greeter/v1/greeter.proto
```

### Options

This tool supports all the same options from `pbjson-build`. For more
information on the effects of these settings, see the related documentation
from that crate:

* `default_package_filename=<value>`: [default_package_filename](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.default_package_filename)
* `extern_path=<proto_path>=<rust_path>`:  [extern_path](https://docs.rs/pbjson-build/latest/pbjson_build/struct.Builder.html#method.extern_path)
* `retain_enum_prefix(=<boolean>)`:  [retain_enum_prefix](https://docs.rs/pbjson-build/latest/pbjson_build/struct.Builder.html#method.retain_enum_prefix)

A note on parameter values:

* `<proto_path>`: Protobuf paths beginning with `.` will be matched from the
  global root (prefix matches). All other paths will be matched as suffix
  matches.
* `(=<boolean>)`: Boolean values may be specified after a parameter, but if
  not, the value is assumed to be `true` by virtue of having listed the
  parameter.

### Usage with _buf_

When used with _buf_, options can be specified in the `buf.gen.yaml` file.
`protoc-gen-prost-serde` should appear as a plugin step after any
`protoc-gen-prost` steps. In addition, the `compile_well_known_types`
and `extern_path=.google.protobuf=::pbjson_types` options should be specified.

```yaml
version: v1
plugins:
  - name: prost
    out: gen
    opt:
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
  - name: prost-serde
    out: gen
```

`protoc-gen-prost-serde` is also compatible with the `protoc-gen-prost-crate`
plugin:

```yaml
version: v1
plugins:
  - name: prost
    out: gen
    opt:
      - compile_well_known_types
      - extern_path=.google.protobuf=::pbjson_types
  - name: prost-serde
    out: gen
  - name: prost-crate
    strategy: all
    out: gen
    opt:
      - no_features
```

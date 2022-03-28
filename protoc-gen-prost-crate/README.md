# protoc-gen-prost-crate

A `protoc` plugin that generates Cargo crates and include files.

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

Ensure that `protoc-gen-prost-crate` has been installed within a directory on your
`$PATH`. Then invoke `protoc` from the command line as follows:

```shell
protoc --prost-crate_out=proto/gen -I proto proto/greeter/v1/greeter.proto
```

### Options

The following options can be specified:

* `default_package_filename=<value>`: This should match the value of the
  main `protoc-gen-prost` step so that the include file references the correct
  output file. (see also [default_package_filename](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.default_package_filename))
* `include_file(=<value>)`: Sets the name for the generated include file. If
  not specified, then the default value depends on whether `gen_crate` is
  specified. If it is, then `<value>` defaults to `src/lib.rs`. Otherwise, it
  defaults to `mod.rs`. The generated include file will add feature markers
   to each produced module to allow for faster compilation and reduced sizes
  for unnecessary features. This can be disabled by specifying `no_features`.
* `gen_crate=<template_path>`: Indicates that a Cargo crate should be generated
  with the manifest based on the template at the path provided. The template
  should include a placeholder to inject the crate features graph unless
  `no_features` is specified. The template generation does not take into
  account crate dependencies. These must be managed separately.
* `no_features(=<boolean>)`: Disables the generation of feature flags in the
  include file or in the Cargo manifest.

A note on parameter values:

* `(=<boolean>)`: Boolean values may be specified after a parameter, but if
  not, the value is assumed to be `true` by virtue of having listed the
  parameter.

### Usage with _buf_

When used with _buf_, options can be specified in the `buf.gen.yaml` file.
This plugin _must_ be run with `strategy: all` in order to have a complete
view of the protobuf schemas and correctly identify dependencies for the
input files.

The following will just produce an include file `mod.rs` in the output `gen`
directory:

```yaml
version: v1
plugins:
  - name: prost-crate
    out: gen
    strategy: all
```

When using the `gen_crate` option, later Rust generators should generate
into the `src` directory which will be created by this plugin:

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
      - file_descriptor_set
```

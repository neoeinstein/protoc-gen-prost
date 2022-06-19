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
  output file. (see also [default package filename] from _prost-build_)
* `include_file(=<value>)`: Sets the name for the generated include file. If
  not specified, then the default value depends on whether `gen_crate` is
  specified. If it is, then `<value>` defaults to `src/lib.rs`. Otherwise, it
  defaults to `mod.rs`. The generated include file will add feature markers
   to each produced module to allow for faster compilation and reduced sizes
  for unnecessary features. This can be disabled by specifying `no_features`.
* `only_include=<proto_path>`: Will only include packages with the specified
  prefix in the generated include file and generated features list, if enabled.
  Paths must be fully-qualified and begin with `.`.
* `gen_crate(=<template_path>`): Indicates that a Cargo crate should be
  generated with the manifest based on the template at the path provided. The
  template should include a placeholder to inject the crate features graph
  unless `no_features` is specified. The template generation does not take
  into account crate dependencies. These must be managed separately. If not
  specified, `Cargo.toml` is assumed. This allows automatic updating of the
  features in the Cargo manifest.
* `no_features(=<boolean>)`: Disables the generation of feature flags in the
  include file or in the Cargo manifest.

[default package filename]: https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.default_package_filename

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
directory without any conditional compilation feature flags:

```yaml
version: v1
plugins:
  - name: prost-crate
    out: gen
    strategy: all
    opt:
      - no_features
```

When using the `gen_crate` option, later Rust generators should generate
into the `src` directory which will be created by this plugin:

```yaml
version: v1
plugins:
  - name: prost
    out: gen/src
    opt:
      - bytes=.
      - file_descriptor_set
  - name: prost-crate
    out: gen
    strategy: all
    opt:
      - gen_crate
```

When working with private, vendored package dependencies, _buf_ tends to
push more files for output than desired. To limit the packages that get
put into the include file, used the `only_include` option:

```yaml
version: v1
plugins:
  - name: prost
    out: gen/src
    opt:
      - bytes=.
      - file_descriptor_set
  - name: prost-crate
    out: gen
    strategy: all
    opt:
      - gen_crate
      - only_include=.my_company
```

The `protoc-gen-prost-crate` plugin is also published on the Buf Schema Registry as
a plugin which you can execute remotely, without needing to explicitly install
this tool. See the [plugin listing][1] to identify the latest published version
for use. Note that the remote plugin form is _not compatible_ with the `gen_crate`
option, as the plugin is executed outside the context of the current file system,
so template information cannot be used. The plugin is referenced as follows:

[1]: https://buf.build/prost/plugins/crate

```yaml
version: v1
plugins:
  - remote: buf.build/prost/plugins/crate:v0.1.6-1
    out: gen
```

## Cargo manifest template

When using the `gen_crate` option, the template specified will be placed in
the output folder. The template should include a commented insertion point
that can be referenced for the placement of the features dependency graph.
The template will be `Cargo.toml` by default, and will do an in-place update
of the `Cargo.toml` file, updating the exposed features to match the generated
code.

`protoc` will insert the computed dependency graph above the insertion point.
If this commented line is not included, then `protoc` will not know where to
place the features list and the insertion will be silently skipped.

```toml
[package]
name = "my-cool-proto-defs"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.10.0"

[features]
default = []
## @@protoc_insertion_point(features)
```

Once generated, the manifest will include a set of features that allow
limiting the compilation to only those modules that are required. Each feature
will automatically activate the relevant features required.

```toml
[package]
name = "my-cool-proto-defs"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.10.0"

[features]
default = []
## @@protoc_deletion_point(features)
## This section is automatically generated by protoc-gen-prost-crate. 
## Changes in this area may be lost on regeneration.
proto_full = ["helloworld_v1", "greeter_v1"]
greeter_v1 = ["helloworld_v1"]
helloworld_v1 = []
## @@protoc_insertion_point(features)
```

## Customizing after generation

In general, I counsel against manually editing the generated code files. Doing
so makes it more difficult to evolve your schema, as regenerating the code
will result in needing to manually edit again in order to re-add the
customizations. Instead, I recommend using a subdirectory to host the generated
code and include that file in `src/lib.rs` as in the `build-with-buf` example.

`src/lib.rs`:
```no_compile
include!("gen/mod.rs");

// Any additional traits or other impls can go here or in other related files.
// These won't be touched by the code generation process.
```

`Cargo.toml` (prior to first generation):
```toml
[package]
name = "my-cool-proto-defs"
version = "0.1.0"
edition = "2021"

[features]
default = ["proto_full"]
## @@protoc_insertion_point(features)

[dependencies]
prost = "0.10.0"
```

`buf.gen.yaml`:
```yaml
version: v1
plugins:
  - name: prost
    out: src
    opt:
      - bytes=.
      - file_descriptor_set
  - name: prost-crate
    out: gen
    strategy: all
    opt:
      - include_file=src/gen/mod.rs
      - gen_crate
```

# Changelog

This changelog is based on the format from [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [2024-02-15]

- `protoc-gen-prost` 0.3.1

### Added

- (prost) Added support for `enable_type_names` ([#85])

[#85]: https://github.com/neoeinstein/protoc-gen-prost/pull/85

## [2024-02-14]

- `protoc-gen-prost` 0.3.0
- `protoc-gen-prost-serde` 0.3.0
- `protoc-gen-prost-crate` 0.4.0
- `protoc-gen-tonic` 0.4.0

### BREAKING CHANGES

- Updated code generation for latest tonic (0.11), prost (0.12), and pbjson (0.6)

### Fixed

- (serde) Fixed support for using BTree maps ([#79])

### Changed

- Updated the versions of `h2` and `rustix` used in example crates

[#79]: https://github.com/neoeinstein/protoc-gen-prost/pull/79

## [2023-07-06]

- `protoc-gen-prost` 0.2.3
- `protoc-gen-tonic` 0.3.0

### BREAKING CHANGES

- (tonic) Updated code generation to support `tonic` 0.9 ([#72])

### Fixed

- (prost) Fixed missing outputs when multiple protos define the same package ([#71])
- (docs) Update documentation for prost remote plugins ([#69])

### Changed

- Updated the version of `h2` used by the example crate ([#65])

Thanks to @abrisco and @pkwarren for their contributions this release

[#65]: https://github.com/neoeinstein/protoc-gen-prost/pull/65
[#69]: https://github.com/neoeinstein/protoc-gen-prost/pull/69
[#71]: https://github.com/neoeinstein/protoc-gen-prost/pull/71
[#72]: https://github.com/neoeinstein/protoc-gen-prost/pull/72

## [2023-03-01]

- `protoc-gen-prost-serde` 0.2.3

### Fixed

- (serde) Handle module paths that contain keywords

## [2023-02-25]

- `protoc-gen-prost` 0.2.2
- `protoc-gen-prost-serde` 0.2.2
- `protoc-gen-tonic` 0.2.2

### Fixed

- (prost) Corrected matching for the file descriptor set option ([#46])

### Added

- (tonic) Add option to disable transport generation ([#43])
- (prost) Support applying message and enum attributes ([#48])
- (serde) Support additional pbjson features ([#50])

### Changed

- Updated various dependencies, prost to 0.11.8 ([#50])

Thanks to @streamingfast, @stan-irl, @horacimacias, and @torkelrogstad for
their contributions in these releases

[#43]: https://github.com/neoeinstein/protoc-gen-prost/pull/43
[#46]: https://github.com/neoeinstein/protoc-gen-prost/pull/46
[#48]: https://github.com/neoeinstein/protoc-gen-prost/pull/48
[#50]: https://github.com/neoeinstein/protoc-gen-prost/pull/50

## [2022-11-13]

- `protoc-gen-prost` 0.2.1
- `protoc-gen-prost-crate` 0.3.1
- `protoc-gen-prost-serde` 0.2.1
- `protoc-gen-tonic` 0.2.1
- `protoc-wkt` 1.0.0+3.20.1

*Note*: `prost-build` v0.11.2 introduces a `println!` that causes output to standard out which `protoc`
does not expect. If you encounter this error, install the binaries with `--locked` or use the remote
plugin. The latest published versions are will only accept `0.11.0` and `0.11.1`. Once this is resolved
above, a new release will be published that again accepts `^0.11.3`.

### Added

- (serde) Add support for `preserve_proto_field_names` option ([#36])
- (wkt) Added new crate `protoc-wkt` for file descriptors ([#37])

### Changed

- Updated prost dependency to 0.11.0 ([#35])
- Update various other internal dependencies ([#35])
- (serde) Updated pbjson dependency to 0.5.1 ([#35])

[#35]: https://github.com/neoeinstein/protoc-gen-prost/pull/35
[#36]: https://github.com/neoeinstein/protoc-gen-prost/pull/36
[#37]: https://github.com/neoeinstein/protoc-gen-prost/pull/37

## [2022-08-08]

- `protoc-gen-prost` 0.2.0
- `protoc-gen-prost-crate` 0.3.0
- `protoc-gen-prost-serde` 0.2.0
- `protoc-gen-tonic` 0.2.0

### Changed

- Updated prost dependency to 0.10.0 ([#23])
- (serde) Updated pbjson dependency to 0.4.0 ([#23])
- (tonic) Updated tonic dependency to 0.8.0 ([#23])

[#23]: https://github.com/neoeinstein/protoc-gen-prost/pull/23

## [2022-07-26]

- `protoc-gen-tonic` 0.1.3

### Fixed

- (tonic) Correct parsing of the `no_include` option ([#17])

[#17]: https://github.com/neoeinstein/protoc-gen-prost/pull/17

## [2022-06-22]

- `protoc-gen-prost-crate` 0.2.0

### Added

- (crate) Add option to specify the package part separator in created features: `package_separator`

### Changed

- (crate) The default package part separator has changed from `_` to `-`

## [2022-06-18]

- `protoc-gen-prost` 0.1.4
- `protoc-gen-prost-crate` 0.1.6
- `protoc-gen-prost-serde` 0.1.1
- `protoc-gen-tonic` 0.1.1

### Added

- (serde, tonic) Added `no_include` option to avoid insertion error in non-chained `protoc` invocations ([#13])
- Add `--version` option when invoking the plugin directly ([#9])

### Changed

- `prost-build` updated to 0.10.4
- `pbjson-build` updated to 0.3.2
- `tonic-build` updated to 0.7.2

### Fixed

- Fixed argument parser to accept `=` in a three-part compiler option ([#12])
- (prost) Properly honor the `file_descriptor_set` option ([#11])

[#9]: https://github.com/neoeinstein/protoc-gen-prost/pull/9
[#11]: https://github.com/neoeinstein/protoc-gen-prost/pull/11
[#12]: https://github.com/neoeinstein/protoc-gen-prost/pull/12
[#13]: https://github.com/neoeinstein/protoc-gen-prost/pull/13

<!-- markdownlint-disable-file MD024 -->

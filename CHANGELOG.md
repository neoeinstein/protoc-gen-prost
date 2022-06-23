# Changelog

This changelog is based on the format from [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

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

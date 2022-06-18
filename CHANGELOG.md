# Changelog

This changelog is based on the format from [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

- (serde, tonic) Added `no_include` option to avoid insertion error in non-chained `protoc` invocations ([#13])
- Add `--version` option when invoking the plugin directly ([#9])

### Fixed

- Fixed argument parser to accept `=` in a three-part compiler option ([#12])
- (prost) Properly honor the `file_descriptor_set` option ([#11])

[#9]: https://github.com/neoeinstein/protoc-gen-prost/pull/9
[#11]: https://github.com/neoeinstein/protoc-gen-prost/pull/11
[#12]: https://github.com/neoeinstein/protoc-gen-prost/pull/12
[#13]: https://github.com/neoeinstein/protoc-gen-prost/pull/13

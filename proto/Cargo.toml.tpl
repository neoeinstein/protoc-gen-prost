[package]
name = "proto"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.10.0"
pbjson-types = "0.2.3"
serde = "1.0"
tonic = { version = "0.6.2", features = ["compression"] }

[features]
default = []
# @@protoc_insertion_point(features)

# Remember to keep an insertion point for features if you are using that functionality.

# We're in the middle of everyone upgrading to Prost! 0.10.0. Until that happens, enjoy these patches.
[patch.crates-io]
pbjson-types = { git = "https://github.com/influxdata/pbjson", branch = "main" }
pbjson-build = { git = "https://github.com/influxdata/pbjson", branch = "main" }
tonic = { git = "https://github.com/neoeinstein/tonic", branch = "prost-0.10.0" }

[workspace]

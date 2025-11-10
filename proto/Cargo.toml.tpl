[package]
name = "proto"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.14.1"
pbjson-types = "0.8.0"
serde = "1.0"
tonic = { version = "0.14.0", features = ["gzip"] }
tonic-prost = "0.14.2"

[features]
default = []
# @@protoc_insertion_point(features)

# Remember to keep an insertion point for features if you are using that functionality.

[workspace]

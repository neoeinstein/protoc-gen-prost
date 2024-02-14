[package]
name = "proto"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.12.0"
pbjson-types = "0.6"
serde = "1.0"
tonic = { version = "0.11", features = ["gzip"] }

[features]
default = []
# @@protoc_insertion_point(features)

# Remember to keep an insertion point for features if you are using that functionality.

[workspace]

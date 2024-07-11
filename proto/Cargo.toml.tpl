[package]
name = "proto"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.13.1"
pbjson-types = "0.7"
serde = "1.0"
tonic = { version = "0.12", features = ["gzip"] }

[features]
default = []
# @@protoc_insertion_point(features)

# Remember to keep an insertion point for features if you are using that functionality.

[workspace]

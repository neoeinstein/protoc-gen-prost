[package]
name = "proto"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.10.0"
pbjson-types = "0.3"
serde = "1.0"
tonic = { version = "0.7", features = ["compression"] }

[features]
default = []
# @@protoc_insertion_point(features)

# Remember to keep an insertion point for features if you are using that functionality.

[workspace]

[package]
name = "proto"
version = "0.1.0"
edition = "2021"

[dependencies]
prost = "0.9.0"
prost-types = "0.9.0"

[features]
{{ features }}
# This is probably not what you want, but you can use this template to
# get started. Remember to keep a spot for `{{ features }}` if you are
# using that functionality.

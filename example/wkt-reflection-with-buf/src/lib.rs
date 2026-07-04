include!("gen/mod.rs");

use anyhow::Context;
use tonic_reflection::server::Builder as ReflectionBuilder;

use protoc_wkt::google::protobuf::FILE_DESCRIPTOR_SET as GOOGLE_PROTOBUF_FILE_DESCRIPTOR_SET;

/// Builds a gRPC reflection service for the generated `example` API.
///
/// When protobuf messages reference well-known types (for example
/// `google.protobuf.Timestamp`) and those types are mapped with
/// `extern_path=.google.protobuf=::pbjson_types`, register the well-known
/// type file descriptor set from the `protoc-wkt` crate in addition to every
/// generated module descriptor set. Omitting either set compiles successfully
/// but reflection fails at runtime.
pub fn reflection_service(
) -> anyhow::Result<
    tonic_reflection::server::v1::ServerReflectionServer<
        impl tonic_reflection::server::v1::ServerReflection,
    >,
> {
    ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(example::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(GOOGLE_PROTOBUF_FILE_DESCRIPTOR_SET)
        .build_v1()
        .context("failed to build gRPC reflection service")
}

#[cfg(test)]
mod tests {
    use super::reflection_service;

    #[test]
    fn reflection_service_builds() {
        reflection_service().expect("reflection service should build");
    }
}

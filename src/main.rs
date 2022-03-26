use prost::Message;
use prost_types::compiler::{code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse};
use protoc_gen_prost::Parameters;
use std::io;
use std::io::{Error, ErrorKind, Result};
use std::io::{Read, Write};

fn main() -> Result<()> {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)?;

    let request = CodeGeneratorRequest::decode(&*buf).map_err(|error| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("invalid FileDescriptorSet: {}", error),
        )
    })?;

    let result = match request.parameter().parse::<Parameters>() {
        Ok(params) => params.run(request.file_to_generate, request.proto_file),
        Err(err) => Err(io::Error::new(io::ErrorKind::InvalidInput, err)),
    };

    let response = match result {
        Ok(file) => CodeGeneratorResponse {
            file,
            supported_features: Some(code_generator_response::Feature::Proto3Optional as u64),
            ..CodeGeneratorResponse::default()
        },
        Err(error) => CodeGeneratorResponse {
            error: Some(error.to_string()),
            supported_features: Some(code_generator_response::Feature::Proto3Optional as u64),
            ..CodeGeneratorResponse::default()
        },
    };

    let mut out = Vec::new();
    response.encode(&mut out).map_err(|error| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("invalid FileDescriptorSet: {}", error),
        )
    })?;
    std::io::stdout().write_all(&out)?;

    Ok(())
}

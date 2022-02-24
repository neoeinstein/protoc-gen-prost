use std::collections::{HashMap, HashSet};
use prost::Message;
use prost_types::compiler::{code_generator_response, CodeGeneratorRequest, CodeGeneratorResponse};
use std::io::{Error, ErrorKind, Result};
use std::io::{Read, Write};
use prost_types::FileDescriptorProto;
use protoc_gen_prost::ident::to_snake;

fn main() -> Result<()> {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf)?;

    let request = CodeGeneratorRequest::decode(&*buf).map_err(|error| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("invalid FileDescriptorSet: {}", error.to_string()),
        )
    })?;

    dbg!(&request.proto_file.len());
    dbg!(&request.file_to_generate);
    dbg!(&request.compiler_version);
    dbg!(&request.parameter);

    let files: HashSet<_> = request.file_to_generate.iter().map(String::as_str).collect();

    let (requests, mut file_maps) = request.proto_file.into_iter().map(|file| {
        let module = module(&file);
        let mut file_name = if module.is_empty() {
            "_".to_string()
        } else {
            module.join(".")
        };

        file_name.push_str(".rs");

        let file_name = files.contains(file.name()).then(|| file_name);

        ((module.clone(), file), (module, file_name))
    }).unzip::<_, _, Vec<_>, HashMap<_, _>>();

    let response = match prost_build::Config::new().generate(requests) {
        Ok(results) => {
            let file = results.into_iter()
                .filter_map(|(module, content)| {
                    file_maps.remove(&module).and_then(|name|
                        name.map(|name| {
                            code_generator_response::File {
                                name: Some(name),
                                content: Some(content),
                                ..code_generator_response::File::default()
                            }
                        })
                    )
                }).collect();
            CodeGeneratorResponse {
                file,
                supported_features: Some(code_generator_response::Feature::Proto3Optional as u64),
                ..CodeGeneratorResponse::default()
            }
        }
        Err(error) => {
            CodeGeneratorResponse {
                error: Some(error.to_string()),
                supported_features: Some(code_generator_response::Feature::Proto3Optional as u64),
                ..CodeGeneratorResponse::default()
            }
        }
    };

    let mut out = Vec::new();
    response.encode(&mut out).map_err(|error| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("invalid FileDescriptorSet: {}", error.to_string()),
        )
    })?;
    std::io::stdout().write_all(&out)?;

    Ok(())
}

type Module = Vec<String>;

fn module(file: &FileDescriptorProto) -> Module {
    file.package()
        .split('.')
        .filter(|s| !s.is_empty())
        .map(to_snake)
        .collect()
}

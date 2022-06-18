#![doc = include_str!("../README.md")]

use self::generator::PbJsonGenerator;
use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use protoc_gen_prost::{Generator, InvalidParameter, ModuleRequestSet, Param, Params};
use std::str;

mod generator;

/// Execute the core _Prost!_ generator from a raw [`CodeGeneratorRequest`]
pub fn execute(raw_request: &[u8]) -> protoc_gen_prost::Result {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;

    let mut builder = params.to_pbjson_builder();
    for file in &request.proto_file {
        builder.register_file_descriptor(file.clone());
    }

    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.default_package_filename.as_deref(),
    )?;

    let files = PbJsonGenerator::new(builder, !params.no_include).generate(&module_request_set)?;

    Ok(files)
}

/// Parameters use to configure [`Generator`]s built into `protoc-gen-prost-serde`
///
/// [`Generator`]: protoc_gen_prost::generators::Generator
#[derive(Debug, Default)]
struct Parameters {
    default_package_filename: Option<String>,
    extern_path: Vec<(String, String)>,
    retain_enum_prefix: bool,
    no_include: bool,
}

impl Parameters {
    fn to_pbjson_builder(&self) -> pbjson_build::Builder {
        let mut builder = pbjson_build::Builder::new();

        for (proto_path, rust_path) in &self.extern_path {
            builder.extern_path(proto_path, rust_path);
        }

        if self.retain_enum_prefix {
            builder.retain_enum_prefix();
        }

        builder
    }
}

impl str::FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret_val = Self::default();
        for param in Params::from_protoc_plugin_opts(s)? {
            match param {
                Param::Parameter {
                    param: "default_package_filename",
                }
                | Param::Value {
                    param: "default_package_filename",
                    ..
                } => ret_val.default_package_filename = param.value().map(|s| s.into_owned()),
                Param::Parameter {
                    param: "retain_enum_prefix",
                }
                | Param::Value {
                    param: "retain_enum_prefix",
                    value: "true",
                } => ret_val.retain_enum_prefix = true,
                Param::Value {
                    param: "retain_enum_prefix",
                    value: "false",
                } => (),
                Param::Parameter {
                    param: "no_include",
                }
                | Param::Value {
                    param: "no_include",
                    value: "true",
                } => ret_val.no_include = true,
                Param::Value {
                    param: "no_include",
                    value: "false",
                } => (),
                Param::KeyValue {
                    param: "extern_path",
                    key: prefix,
                    value: module,
                } => ret_val.extern_path.push((prefix.to_string(), module)),
                _ => return Err(InvalidParameter::from(param)),
            }
        }

        Ok(ret_val)
    }
}

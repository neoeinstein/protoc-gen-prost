#![doc = include_str!("../README.md")]

use std::str;

use prost::Message;
use prost_types::{compiler::CodeGeneratorRequest, FileDescriptorSet};
use protoc_gen_prost::{Generator, InvalidParameter, ModuleRequestSet, Param, Params};

use self::generator::PrutoipaGenerator;

mod generator;

/// Execute the core _Prost!_ generator from a raw [`CodeGeneratorRequest`]
pub fn execute(raw_request: &[u8]) -> protoc_gen_prost::Result {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;

    let mut file_descriptor_set = FileDescriptorSet { file: Vec::new() };
    for file in &request.proto_file {
        file_descriptor_set.file.push(file.clone());
    }

    let mut builder = prutoipa_build::Builder::new();
    builder.register_descriptors(file_descriptor_set)?;

    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.default_package_filename.as_deref(),
    )?;

    let files =
        PrutoipaGenerator::new(builder, !params.no_include).generate(&module_request_set)?;

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
    preserve_proto_field_names: bool,
    ignore_unknown_fields: bool,
    emit_fields: bool,
    use_integers_for_enums: bool,
    no_include: bool,
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
                    param: "preserve_proto_field_names",
                }
                | Param::Value {
                    param: "preserve_proto_field_names",
                    value: "true",
                } => ret_val.preserve_proto_field_names = true,
                Param::Value {
                    param: "preserve_proto_field_names",
                    value: "false",
                } => (),
                Param::Parameter {
                    param: "ignore_unknown_fields",
                }
                | Param::Value {
                    param: "ignore_unknown_fields",
                    value: "true",
                } => ret_val.ignore_unknown_fields = true,
                Param::Parameter {
                    param: "emit_fields",
                }
                | Param::Value {
                    param: "emit_fields",
                    value: "true",
                } => ret_val.emit_fields = true,
                Param::Parameter {
                    param: "use_integers_for_enums",
                }
                | Param::Value {
                    param: "use_integers_for_enums",
                    value: "true",
                } => ret_val.use_integers_for_enums = true,
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

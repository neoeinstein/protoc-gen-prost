#![doc = include_str!("../README.md")]

use std::str;

use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use protoc_gen_prost::{Generator, InvalidParameter, ModuleRequestSet, Param, Params};

use self::generator::PbJsonGenerator;

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
        params.flat_output_dir,
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
    preserve_proto_field_names: bool,
    ignore_unknown_fields: bool,
    emit_fields: bool,
    use_integers_for_enums: bool,
    no_include: bool,
    btree_map: Vec<String>,
    flat_output_dir: bool,
    exclude: Vec<String>,
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

        if self.preserve_proto_field_names {
            builder.preserve_proto_field_names();
        }

        if self.ignore_unknown_fields {
            builder.ignore_unknown_fields();
        }

        if self.emit_fields {
            builder.emit_fields();
        }

        if self.use_integers_for_enums {
            builder.use_integers_for_enums();
        }

        if !self.btree_map.is_empty() {
            builder.btree_map(self.btree_map.clone());
        }

        builder.btree_map(self.btree_map.clone());

        if !self.exclude.is_empty() {
            builder.exclude(self.exclude.clone());
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
                Param::Value {
                    param: "btree_map",
                    value,
                } => ret_val.btree_map.push(value.to_string()),
                Param::Parameter {
                    param: "flat_output_dir",
                }
                | Param::Value {
                    param: "flat_output_dir",
                    value: "true",
                } => ret_val.flat_output_dir = true,
                Param::Value {
                    param: "flat_output_dir",
                    value: "false",
                } => (),
                Param::Value {
                    param: "exclude",
                    value: prefix,
                } => ret_val.exclude.push(prefix.to_string()),
                _ => return Err(InvalidParameter::from(param)),
            }
        }

        Ok(ret_val)
    }
}

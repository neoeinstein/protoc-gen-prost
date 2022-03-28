#![doc = include_str!("../README.md")]

use self::generator::PbJsonGenerator;
use once_cell::sync::Lazy;
use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use protoc_gen_prost::{Generator, ModuleRequestSet};
use std::{fmt, str};

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

    let files = PbJsonGenerator::new(builder).generate(&module_request_set)?;

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
}

static PARAMETER: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(
        r"(?:(?P<param>[^,=]+)(?:=(?P<key>[^,=]+)(?:=(?P<value>(?:[^,=\\]|\\,|\\)+))?)?)",
    )
    .unwrap()
});

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
        for capture in PARAMETER.captures_iter(s) {
            let param = capture
                .get(1)
                .expect("any captured group will at least have the param name")
                .as_str()
                .trim();

            let key = capture.get(2).map(|m| m.as_str());
            let value = capture.get(3).map(|m| m.as_str());

            match (param, key, value) {
                ("default_package_filename", value, None) => {
                    ret_val.default_package_filename = value.map(|s| s.to_string())
                }
                ("retain_enum_prefix", Some("true") | None, None) => {
                    ret_val.retain_enum_prefix = true
                }
                ("retain_enum_prefix", Some("false"), None) => (),
                ("extern_path", Some(prefix), Some(module)) => ret_val
                    .extern_path
                    .push((prefix.to_string(), module.to_string())),
                _ => {
                    return Err(InvalidParameter(
                        capture.get(0).unwrap().as_str().to_string(),
                    ))
                }
            }
        }

        Ok(ret_val)
    }
}

/// An invalid parameter
#[derive(Debug)]
struct InvalidParameter(String);

impl fmt::Display for InvalidParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid parameter: ")?;
        f.write_str(&self.0)
    }
}

impl std::error::Error for InvalidParameter {}

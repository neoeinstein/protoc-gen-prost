#![doc = include_str!("../README.md")]

use self::generator::TonicGenerator;
use self::resolver::Resolver;
use once_cell::sync::Lazy;
use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use protoc_gen_prost::{Generator, ModuleRequestSet};
use std::{fmt, str};
use tonic_build::Attributes;

mod generator;
mod resolver;
mod util;

/// Execute the core _Prost!_ generator from a raw [`CodeGeneratorRequest`]
pub fn execute(raw_request: &[u8]) -> protoc_gen_prost::Result {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;

    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.default_package_filename.as_deref(),
    )?;

    let resolver = Resolver::new(params.extern_path, params.compile_well_known_types);
    let mut generator = TonicGenerator {
        resolver,
        generate_server: !params.no_server,
        generate_client: !params.no_client,
        server_attributes: params.server_attributes,
        client_attributes: params.client_attributes,
        emit_package: !params.disable_package_emission,
    };

    let files = generator.generate(&module_request_set)?;

    Ok(files)
}

/// Parameters use to configure [`Generator`]s built into `protoc-gen-prost-serde`
///
/// [`Generator`]: protoc_gen_prost::generators::Generator
#[derive(Debug, Default)]
struct Parameters {
    default_package_filename: Option<String>,
    extern_path: Vec<(String, String)>,
    server_attributes: Attributes,
    client_attributes: Attributes,
    compile_well_known_types: bool,
    disable_package_emission: bool,
    no_server: bool,
    no_client: bool,
}

static PARAMETER: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(
        r"(?:(?P<param>[^,=]+)(?:=(?P<key>[^,=]+)(?:=(?P<value>(?:[^,=\\]|\\,|\\=|\\\\)+))?)?)",
    )
    .unwrap()
});

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
            let value = capture.get(3).map(|m| {
                m.as_str()
                    .replace(r"\,", r",")
                    .replace(r"\=", r"=")
                    .replace(r"\\", r"\")
            });

            match (param, key, value) {
                ("default_package_filename", value, None) => {
                    ret_val.default_package_filename = value.map(|s| s.to_string())
                }
                ("extern_path", Some(prefix), Some(module)) => {
                    ret_val.extern_path.push((prefix.to_string(), module))
                }
                ("compile_well_known_types", Some("true") | None, None) => {
                    ret_val.compile_well_known_types = true
                }
                ("compile_well_known_types", Some("false"), None) => (),
                ("disable_package_emission", Some("true") | None, None) => {
                    ret_val.disable_package_emission = true
                }
                ("disable_package_emission", Some("false"), None) => (),
                ("no_server", Some("true") | None, None) => ret_val.no_server = true,
                ("no_server", Some("false"), None) => (),
                ("no_client", Some("true") | None, None) => ret_val.no_client = true,
                ("no_client", Some("false"), None) => (),
                ("client_mod_attribute", Some(prefix), Some(attribute)) => {
                    ret_val.client_attributes.push_mod(prefix, attribute)
                }
                ("client_attribute", Some(prefix), Some(attribute)) => {
                    ret_val.client_attributes.push_struct(prefix, attribute)
                }
                ("server_mod_attribute", Some(prefix), Some(attribute)) => {
                    ret_val.server_attributes.push_mod(prefix, attribute)
                }
                ("server_attribute", Some(prefix), Some(attribute)) => {
                    ret_val.server_attributes.push_struct(prefix, attribute)
                }
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

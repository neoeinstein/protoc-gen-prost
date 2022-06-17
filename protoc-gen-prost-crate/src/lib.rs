#![doc = include_str!("../README.md")]

use self::generator::{CargoCrateGenerator, IncludeFileGenerator};
use once_cell::sync::Lazy;
use prost::Message;

use prost_types::compiler::CodeGeneratorRequest;

use protoc_gen_prost::{Generator, ModuleRequestSet, Result};

use crate::generator::FeaturesGenerator;
use std::rc::Rc;
use std::{fmt, str};

mod generator;

/// Execute the crate file generator from an encoded [`CodeGeneratorRequest`]
pub fn execute(raw_request: &[u8]) -> Result {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;

    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.default_package_filename.as_deref(),
    )?;

    let include_filename = if matches!(params.gen_crate, Some(_)) {
        params.include_file.as_deref().unwrap_or("src/lib.rs")
    } else {
        params.include_file.as_deref().unwrap_or("mod.rs")
    };

    let limiter = Rc::new(params.only_include);

    let include_file_generator = IncludeFileGenerator::new(include_filename, limiter.clone());
    let cargo_crate_generator = params
        .gen_crate
        .as_ref()
        .map(|o| CargoCrateGenerator::new(o.as_deref()));
    let features_generator =
        (!params.no_features).then(|| FeaturesGenerator::new(include_filename, limiter.clone()));

    let files = include_file_generator
        .chain(cargo_crate_generator)
        .chain(features_generator)
        .generate(&module_request_set)?;

    Ok(files)
}

/// Parameters use to configure [`Generator`]s built into `protoc-gen-prost`
///
/// [`Generator`]: crate::Generator
#[derive(Debug, Default)]
struct Parameters {
    /// The default package filename to use when no package is specified
    default_package_filename: Option<String>,

    /// Whether to generate an include file with an optional filename
    include_file: Option<String>,

    /// A path to a template for generating a Rust crate
    gen_crate: Option<Option<String>>,

    /// A path to a template for generating a Rust crate
    no_features: bool,

    /// Limit generation of includes to packages in this list
    only_include: PackageLimiter,
}

static PARAMETER: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(
        r"(?:(?P<param>[^,=]+)(?:=(?P<key>[^,=]+)(?:=(?P<value>(?:[^,=\\]|\\,|\\)+))?)?)",
    )
    .unwrap()
});

impl str::FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
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
                    ret_val.default_package_filename = value.map(ToOwned::to_owned)
                }
                ("include_file", Some(filename), None) => {
                    ret_val.include_file = Some(filename.to_owned())
                }
                ("only_include", Some(package), None) => {
                    if ret_val.only_include.push(package.to_owned()).is_err() {
                        return Err(InvalidParameter(format!(
                            "proto paths must begin with `.`: {}",
                            capture.get(0).unwrap().as_str()
                        )));
                    }
                }
                ("gen_crate", template, None) => {
                    ret_val.gen_crate = Some(template.map(ToOwned::to_owned))
                }
                ("no_features", Some("true") | None, None) => ret_val.no_features = true,
                ("no_features", Some("false"), None) => (),
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

#[derive(Debug, Default)]
struct PackageLimiter {
    include_prefixes: Vec<String>,
}

impl PackageLimiter {
    fn push(&mut self, package: String) -> std::result::Result<(), ()> {
        if package.starts_with('.') {
            let mut prefix = package;
            prefix.remove(0);
            prefix.push('.');
            self.include_prefixes.push(prefix);
            Ok(())
        } else {
            Err(())
        }
    }

    fn is_allowed(&self, package: &str) -> bool {
        if self.include_prefixes.is_empty() {
            true
        } else {
            let package = package.strip_prefix('.').unwrap_or(package);

            self.include_prefixes
                .iter()
                .any(|prefix| package == &prefix[..prefix.len() - 1] || package.starts_with(prefix))
        }
    }
}

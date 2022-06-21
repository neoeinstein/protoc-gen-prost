#![doc = include_str!("../README.md")]

use self::generator::{CargoCrateGenerator, IncludeFileGenerator};
use prost::Message;

use prost_types::compiler::CodeGeneratorRequest;

use protoc_gen_prost::{Generator, InvalidParameter, ModuleRequestSet, Param, Params, Result};

use crate::generator::FeaturesGenerator;
use std::rc::Rc;
use std::str;

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
    let package_separator = params.package_separator.as_deref().unwrap_or("_");

    let limiter = Rc::new(params.only_include);

    let include_file_generator = IncludeFileGenerator::new(include_filename, limiter.clone());
    let cargo_crate_generator = params
        .gen_crate
        .as_ref()
        .map(|o| CargoCrateGenerator::new(o.as_deref()));
    let features_generator = (!params.no_features)
        .then(|| FeaturesGenerator::new(include_filename, package_separator, limiter.clone()));

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

    /// Replace the `.` separator in package names to this character for cargo feature flags.
    /// Defaults to `_` for compatibility with crates.io (see
    /// [the documentation](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section)
    /// for more details).
    ///
    /// It is recommended to use `-` or `+` as a separator to be both compatible with crates.io and
    /// avoid any conflict between pacakges with similar names (such as `foo.bar` and `foo_bar`).
    ///
    /// Allowed characters are `-`, `+`, `_`, `.`.
    package_separator: Option<String>,
}

impl str::FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ret_val = Self::default();
        for param in Params::from_protoc_plugin_opts(s)? {
            match param {
                Param::Parameter {
                    param: "default_package_filename",
                }
                | Param::Value {
                    param: "default_package_filename",
                    ..
                } => ret_val.default_package_filename = param.value().map(|v| v.into_owned()),
                Param::Value {
                    param: "include_file",
                    value: filename,
                } => ret_val.include_file = Some(filename.to_owned()),
                Param::Value {
                    param: "only_include",
                    value: package,
                } => {
                    if ret_val.only_include.push(package.to_owned()).is_err() {
                        return Err(InvalidParameter::new(format!(
                            "proto paths must begin with `.`: only_include={package}",
                        )));
                    }
                }
                Param::Parameter { param: "gen_crate" }
                | Param::Value {
                    param: "gen_crate", ..
                } => ret_val.gen_crate = Some(param.value().map(|t| t.into_owned())),
                Param::Parameter {
                    param: "no_features",
                }
                | Param::Value {
                    param: "no_features",
                    value: "true",
                } => ret_val.no_features = true,
                Param::Value {
                    param: "no_features",
                    value: "false",
                } => (),
                Param::Value {
                    param: "package_separator",
                    value: value @ ("." | "-" | "+" | "_"),
                } => ret_val.package_separator = Some(value.to_string()),
                _ => return Err(InvalidParameter::from(param)),
            }
        }

        Ok(ret_val)
    }
}

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

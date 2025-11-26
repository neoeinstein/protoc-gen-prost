mod generator;

use crate::generator::ProstValidateGenerator;
use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use protoc_gen_prost::{
    Generator, InvalidParameter, ModuleRequestSet, Param, Params, ProstParameters,
};
use std::str::FromStr;

/// Parameters use to configure [`Generator`]s built into `protoc-gen-prost-validate`
///
/// [`Generator`]: crate::Generator
#[derive(Debug, Default)]
pub struct Parameters {
    /// Prost parameters, used to generate [`prost_build::Config`]
    pub prost: ProstParameters,

    /// Whether to include the `include!` directive in the prost generated file
    pub no_include: bool,
}

impl FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ret_val = Self::default();
        for param in Params::from_protoc_plugin_opts(s)? {
            if let Err(param) = ret_val.prost.try_handle_parameter(param) {
                match param {
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
                    _ => return Err(InvalidParameter::from(param)),
                }
            }
        }

        Ok(ret_val)
    }
}

/// Execute the core _Prost!_ generator from a raw [`CodeGeneratorRequest`]
pub fn execute(raw_request: &[u8]) -> protoc_gen_prost::Result {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;
    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.prost.default_package_filename(),
        params.prost.flat_output_dir,
    )?;
    let files = ProstValidateGenerator::new(params.prost.to_prost_config(), !params.no_include)
        .generate(&module_request_set)?;
    Ok(files)
}

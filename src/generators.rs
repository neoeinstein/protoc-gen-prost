//! Code generator modules

use crate::ModuleRequestSet;
use std::ops::DerefMut;

use prost_types::compiler::code_generator_response::{Feature, File};
use prost_types::compiler::CodeGeneratorResponse;

pub mod core;
pub mod file_descriptor_set;
pub mod include_file;

/// A code generation result
pub type Result = std::result::Result<Vec<File>, Error>;
/// A code generation error
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Extension function to assist in converting [`Result`] into a [`CodeGeneratorResponse`]
pub trait GeneratorResultExt {
    /// Unwrap a [`Result`], producing the relevant [`CodeGeneratorResponse`]
    fn unwrap_codegen_response(self) -> CodeGeneratorResponse;
}

impl GeneratorResultExt for Result {
    fn unwrap_codegen_response(self) -> CodeGeneratorResponse {
        match self {
            Ok(file) => CodeGeneratorResponse {
                file,
                supported_features: Some(Feature::Proto3Optional as u64),
                ..Default::default()
            },
            Err(error) => error_to_codegen_response(&*error),
        }
    }
}

fn error_to_codegen_response(error: &dyn std::error::Error) -> CodeGeneratorResponse {
    CodeGeneratorResponse {
        error: Some(error.to_string()),
        supported_features: Some(Feature::Proto3Optional as u64),
        ..Default::default()
    }
}

/// A code generator
pub trait Generator {
    /// Generate one or more files based on the input request
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result;
}

impl<I> Generator for I
where
    I: Iterator,
    I::Item: DerefMut<Target = dyn Generator>,
{
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let mut file = Vec::new();
        for mut generator in self {
            let generated = generator.deref_mut().generate(module_request_set)?;
            file.extend(generated);
        }
        Ok(file)
    }
}

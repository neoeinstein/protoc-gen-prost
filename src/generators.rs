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

/// A code generator
pub trait Generator {
    /// Generate one or more files based on the input request
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result;
}

/// Extension trait for collecting code generation [`Result`]s into a [`CodeGeneratorResponse`]
pub trait GeneratorPipeline {
    /// Collect code generation results with the specified input request set
    fn collect_code_generator_response(
        &mut self,
        module_request_set: &ModuleRequestSet,
    ) -> CodeGeneratorResponse;
}

impl<I> GeneratorPipeline for I
where
    I: Iterator,
    I::Item: DerefMut<Target = dyn Generator>,
{
    fn collect_code_generator_response(
        &mut self,
        module_request_set: &ModuleRequestSet,
    ) -> CodeGeneratorResponse {
        let mut file = Vec::new();
        for mut generator in self {
            match generator.deref_mut().generate(module_request_set) {
                Ok(generated) => file.extend(generated),
                Err(err) => {
                    return CodeGeneratorResponse {
                        error: Some(err.to_string()),
                        supported_features: Some(Feature::Proto3Optional as u64),
                        ..Default::default()
                    }
                }
            }
        }
        CodeGeneratorResponse {
            file,
            supported_features: Some(Feature::Proto3Optional as u64),
            ..Default::default()
        }
    }
}

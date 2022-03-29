use prost_types::compiler::code_generator_response::File;

use protoc_gen_prost::{Generator, ModuleRequestSet, Result};
use std::fs;

pub(crate) struct CargoCrateGenerator {
    manifest_template_path: String,
}

impl CargoCrateGenerator {
    pub(crate) fn new(manifest_template_path: String) -> Self {
        Self {
            manifest_template_path,
        }
    }
}

impl Generator for CargoCrateGenerator {
    fn generate(&mut self, _: &ModuleRequestSet) -> Result {
        let manifest_template = fs::read_to_string(&self.manifest_template_path)?;

        Ok(vec![File {
            name: Some("Cargo.toml".to_string()),
            content: Some(manifest_template),
            ..File::default()
        }])
    }
}

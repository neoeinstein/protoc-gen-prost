use std::{fs, io, io::BufRead};

use prost_types::compiler::code_generator_response::File;
use protoc_gen_prost::{Generator, ModuleRequestSet, Result};

pub(crate) struct CargoCrateGenerator<'a> {
    manifest_template_path: Option<&'a str>,
}

impl<'a> CargoCrateGenerator<'a> {
    pub(crate) fn new(manifest_template_path: Option<&'a str>) -> Self {
        Self {
            manifest_template_path,
        }
    }
}

const DEFAULT_TEMPLATE: &str = "Cargo.toml";

impl<'a> Generator for CargoCrateGenerator<'a> {
    fn generate(&mut self, _: &ModuleRequestSet) -> Result {
        let template_file = fs::OpenOptions::new()
            .read(true)
            .open(self.manifest_template_path.unwrap_or(DEFAULT_TEMPLATE))?;
        let buffered = io::BufReader::new(template_file);
        let mut manifest_template = String::new();
        let mut drop = false;

        for line in buffered.lines() {
            let line = line?;
            if drop {
                drop = !line.contains("@@protoc_insertion_point(features)");
            } else {
                drop = line.contains("@@protoc_deletion_point(features)");
            }

            if drop {
                continue;
            }

            manifest_template.push_str(&line);
            manifest_template.push('\n');
        }

        Ok(vec![File {
            name: Some("Cargo.toml".to_string()),
            content: Some(manifest_template),
            ..File::default()
        }])
    }
}

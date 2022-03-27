use crate::generators::{Generator, Result};
use crate::ModuleRequestSet;
use prost_build::Module;
use prost_types::compiler::code_generator_response::File;

pub struct IncludeFileGenerator {
    filename: Option<String>,
}

impl IncludeFileGenerator {
    pub fn new(filename: Option<String>) -> Self {
        Self { filename }
    }
}

impl Generator for IncludeFileGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let parts: [String; 0] = [];
        let root = Module::from_parts(parts);
        let mut last = &root;
        let mut last_prefix = 0;
        let mut indent = String::new();
        let mut content = String::new();

        let _: () = module_request_set
            .requests()
            .filter_map(|(module, request)| {
                let filename = request.output_filename()?;

                let (down, prefix) = difference(&last, module);

                for _ in 0..down {
                    indent.truncate(indent.len().checked_sub(4).expect("indent underflow"));
                    content.push_str(&indent);
                    content.push_str("}\n");
                }

                for module_part in module.parts().skip(prefix) {
                    content.push_str(&indent);
                    content.push_str("pub mod ");
                    content.push_str(module_part);
                    content.push_str(" {\n");
                    indent.push_str("    ");
                }

                content.push_str(&indent);
                content.push_str("include!(\"");
                content.push_str(filename);
                content.push_str("\");\n");

                content.push_str(&indent);
                content.push_str("// @@protoc_insertion_point(");
                content.push_str(request.proto_package_name());
                content.push_str(")\n");

                last = module;
                last_prefix = prefix;

                Some(())
            })
            .collect();

        for _ in 0..=last_prefix {
            indent.truncate(indent.len().checked_sub(4).expect("indent underflow"));
            content.push_str(&indent);
            content.push_str("}\n");
        }

        let file = File {
            name: Some(self.filename.as_deref().unwrap_or("lib.rs").to_string()),
            content: Some(content),
            ..File::default()
        };

        Ok(vec![file])
    }
}

fn difference(left: &Module, right: &Module) -> (usize, usize) {
    let mut left_parts = left.parts();
    let mut right_parts = right.parts();

    let mut prefix = 0;

    loop {
        match (left_parts.next(), right_parts.next()) {
            (Some(left), Some(right)) if left == right => prefix += 1,
            (Some(_), Some(_)) => return (left_parts.count() + 1, prefix),
            (Some(_), None) => return (left_parts.count() + 1, prefix),
            (None, Some(_)) => return (0, prefix),
            (None, None) => return (0, prefix),
        }
    }
}

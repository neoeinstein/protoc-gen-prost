use prost_build::Module;
use prost_types::compiler::code_generator_response::File;
use protoc_gen_prost::{Generator, ModuleRequestSet, Result};

const GENERATED_HEADER: &str = "// @generated\n";

pub struct PrutoipaGenerator {
    builder: prutoipa_build::Builder,
    insert_include: bool,
}

impl PrutoipaGenerator {
    pub fn new(builder: prutoipa_build::Builder, insert_include: bool) -> Self {
        Self {
            builder,
            insert_include,
        }
    }
}

impl Generator for PrutoipaGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let results = self.builder.generate(|_| Ok(Vec::new()))?;

        results
            .into_iter()
            .filter_map(|(package, bytes)| {
                let request = module_request_set.for_module(
                    &Module::from_protobuf_package_name(&package.to_string().replace("r#", "")),
                )?;

                let output_filename = format!("{}.utoipa.rs", request.proto_package_name());

                let mut res = Vec::with_capacity(2);

                if self.insert_include {
                    res.push(request.append_to_file(|buf| {
                        buf.push_str("include!(\"");
                        buf.push_str(&output_filename);
                        buf.push_str("\");\n");
                    })?);
                }

                let mut content = String::with_capacity(bytes.len() + GENERATED_HEADER.len());
                content.push_str(GENERATED_HEADER);
                content.push_str(
                    std::str::from_utf8(&bytes).expect("prutoipa build produced non UTF-8 data"),
                );

                res.push(File {
                    name: Some(output_filename),
                    content: Some(content),
                    ..File::default()
                });

                Some(res)
            })
            .flatten()
            .map(Ok)
            .collect()
    }
}

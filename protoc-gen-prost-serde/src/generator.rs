use prost_build::Module;
use prost_types::compiler::code_generator_response::File;
use protoc_gen_prost::{Generator, ModuleRequestSet, Result};

const GENERATED_HEADER: &str = "// @generated\n";

pub struct PbJsonGenerator {
    builder: pbjson_build::Builder,
    prefixes: Vec<String>,
}

impl Generator for PbJsonGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let results = self.builder.generate(&self.prefixes, |_| Ok(Vec::new()))?;

        results
            .into_iter()
            .filter_map(|(package, bytes)| {
                let request = module_request_set
                    .for_module(&Module::from_protobuf_package_name(&package.to_string()))?;

                let output_filename = format!("{}.serde.rs", request.proto_package_name());

                let insertion = request.append_to_file(|buf| {
                    buf.push_str("include!(\"");
                    buf.push_str(&output_filename);
                    buf.push_str("\");\n");
                })?;

                let mut content = String::with_capacity(bytes.len() + GENERATED_HEADER.len());
                content.push_str(GENERATED_HEADER);
                content.push_str(
                    std::str::from_utf8(&bytes).expect("pbjson build produced non UTF-8 data"),
                );

                let data = File {
                    name: Some(output_filename),
                    content: Some(content),
                    ..File::default()
                };

                Some([data, insertion])
            })
            .flatten()
            .map(Ok)
            .collect()
    }
}

impl PbJsonGenerator {
    pub fn new(builder: pbjson_build::Builder) -> Self {
        Self {
            builder,
            prefixes: vec![".".to_owned()],
        }
    }
}

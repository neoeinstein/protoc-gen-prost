use crate::generators::Generator;
use crate::ModuleRequestSet;
use prost_build::Module;
use prost_types::compiler::code_generator_response::File;

pub struct CoreProstGenerator {
    config: prost_build::Config,
}

impl Generator for CoreProstGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> super::Result {
        let prost_requests = module_request_set
            .requests()
            .flat_map(|(module, request)| {
                request
                    .files
                    .iter()
                    .map(|proto| (module.clone(), proto.clone()))
            })
            .collect();

        let files = self
            .config
            .generate(prost_requests)?
            .into_iter()
            .filter_map(|(module, content)| {
                Self::content_to_file(module, content, module_request_set)
            })
            .collect();

        Ok(files)
    }
}

impl CoreProstGenerator {
    pub fn new(config: prost_build::Config) -> Self {
        Self { config }
    }

    fn content_to_file(
        module: Module,
        content: String,
        module_requests: &ModuleRequestSet,
    ) -> Option<File> {
        let request = module_requests
            .for_module(&module)
            .expect("generated module that somehow wasn't in the original requests…");

        request.write_to_file(move |buffer| {
            let _ = std::mem::replace(buffer, content);

            buffer.push_str("// @@protoc_insertion_point(");
            buffer.push_str(request.proto_package_name());
            buffer.push_str(")\n");
        })
    }
}

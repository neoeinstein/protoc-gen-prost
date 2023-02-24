use prost_build::{Module, Service};
use prost_types::{
    compiler::code_generator_response::File, FileDescriptorProto, ServiceDescriptorProto,
};
use protoc_gen_prost::{Generator, ModuleRequest, ModuleRequestSet, Result};
use tonic_build::Attributes;

use crate::{resolver::Resolver, util};

pub(crate) struct TonicGenerator {
    pub(crate) resolver: Resolver,
    pub(crate) generate_server: bool,
    pub(crate) generate_client: bool,
    pub(crate) generate_transport: bool,
    pub(crate) server_attributes: Attributes,
    pub(crate) client_attributes: Attributes,
    pub(crate) emit_package: bool,
    pub(crate) insert_include: bool,
}

impl Generator for TonicGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        module_request_set
            .requests()
            .filter_map(|(module, request)| self.handle_module_request(module, request))
            .flatten()
            .map(Ok)
            .collect()
    }
}

impl TonicGenerator {
    fn handle_module_request(&self, module: &Module, request: &ModuleRequest) -> Option<Vec<File>> {
        const PROTO_PATH: &str = "super";

        let output_filename = format!("{}.tonic.rs", request.proto_package_name());

        let services = request
            .files()
            .into_iter()
            .flat_map(|file| {
                file.service
                    .iter()
                    .enumerate()
                    .filter_map(|(service_index, descriptor)| {
                        self.prepare_service(module, file, descriptor, service_index)
                    })
                    .flat_map(|service| {
                        let client = self.generate_client.then(|| {
                            tonic_build::CodeGenBuilder::new()
                                .emit_package(self.emit_package)
                                .build_transport(self.generate_transport)
                                .compile_well_known_types(self.resolver.compile_well_known_types())
                                .attributes(self.client_attributes.clone())
                                .generate_client(&service, PROTO_PATH)
                        });
                        let server = self.generate_server.then(|| {
                            tonic_build::CodeGenBuilder::new()
                                .emit_package(self.emit_package)
                                .build_transport(self.generate_transport)
                                .compile_well_known_types(self.resolver.compile_well_known_types())
                                .attributes(self.server_attributes.clone())
                                .generate_server(&service, PROTO_PATH)
                        });

                        client.into_iter().chain(server)
                    })
            })
            .reduce(|mut l, r| {
                l.extend(r);
                l
            })
            .unwrap_or_default();

        if services.is_empty() {
            None
        } else {
            let mut res = Vec::with_capacity(2);

            let file = syn::parse2(services).expect("valid rust file");

            if self.insert_include {
                res.push(request.append_to_file(|buf| {
                    buf.push_str("include!(\"");
                    buf.push_str(&output_filename);
                    buf.push_str("\");\n");
                })?);
            }

            res.push(File {
                name: Some(output_filename),
                content: Some(format!("// @generated\n{}", prettyplease::unparse(&file))),
                ..File::default()
            });

            Some(res)
        }
    }

    fn prepare_service(
        &self,
        module: &Module,
        file: &FileDescriptorProto,
        descriptor: &ServiceDescriptorProto,
        service_index: usize,
    ) -> Option<Service> {
        let comments = util::get_service_comments(file, service_index);

        let methods = descriptor
            .method
            .iter()
            .enumerate()
            .map(|(method_index, m)| {
                let comments = util::get_method_comments(file, service_index, method_index);
                prost_build::Method {
                    name: util::to_snake(m.name()),
                    proto_name: m.name().to_owned(),
                    comments,
                    input_type: self.resolver.resolve_ident(module, m.input_type()),
                    output_type: self.resolver.resolve_ident(module, m.output_type()),
                    input_proto_type: m.input_type().to_string(),
                    output_proto_type: m.output_type().to_string(),
                    options: m.options.clone().unwrap_or_default(),
                    client_streaming: m.client_streaming(),
                    server_streaming: m.server_streaming(),
                }
            });

        let service = prost_build::Service {
            package: file.package().to_owned(),
            comments,
            methods: methods.collect(),
            name: util::to_upper_camel(descriptor.name()),
            proto_name: descriptor.name().to_owned(),
            options: descriptor.options.clone().unwrap_or_default(),
        };

        Some(service)
    }
}

use prost::Message;
use prost_types::compiler::CodeGeneratorRequest;
use protoc_gen_prost::generators::core::CoreProstGenerator;
use protoc_gen_prost::generators::file_descriptor_set::FileDescriptorSetGenerator;
use protoc_gen_prost::generators::include_file::IncludeFileGenerator;
use protoc_gen_prost::generators::{Generator, GeneratorResultExt};
use protoc_gen_prost::{generators, ModuleRequestSet, Parameters};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    let response = inner(buf.as_slice()).unwrap_codegen_response();

    buf.clear();
    response.encode(&mut buf).expect("error encoding response");
    io::stdout().write_all(&buf)?;

    Ok(())
}

fn inner(raw_request: &[u8]) -> generators::Result {
    let request = CodeGeneratorRequest::decode(raw_request)?;
    let params = request.parameter().parse::<Parameters>()?;

    let module_request_set = ModuleRequestSet::new(
        request.file_to_generate,
        request.proto_file,
        raw_request,
        params.prost.default_package_filename(),
    )?;

    let generators: Vec<Box<dyn Generator>> = if let Some(include_file) = params.include_file {
        let gen = IncludeFileGenerator::new(include_file);

        vec![Box::new(gen)]
    } else {
        let core = CoreProstGenerator::new(params.prost.to_prost_config());
        let fds = FileDescriptorSetGenerator;

        vec![Box::new(core), Box::new(fds)]
    };

    let files = generators.into_iter().generate(&module_request_set)?;

    Ok(files)
}

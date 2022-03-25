use prost_build::Module;
use prost_types::compiler::code_generator_response;
use prost_types::FileDescriptorProto;
use std::collections::{HashMap, HashSet};
use std::{fmt, io, str};

#[derive(Debug, Default)]
pub struct Parameters {
    generate_serde: bool,
}

macro_rules! module_from_package {
    ($package: ident) => {
        Module::from_parts($package.path().into_iter().map(|t| t.to_string()))
    };
}

impl Parameters {
    pub fn run<I>(
        &self,
        file_to_generate: I,
        proto_file: Vec<FileDescriptorProto>,
    ) -> io::Result<Vec<code_generator_response::File>>
    where
        I: IntoIterator<Item = String>,
    {
        let files: HashSet<_> = file_to_generate.into_iter().collect();

        let (requests, mut file_maps) = proto_file
            .iter()
            .map(|file| {
                let module = Module::from_protobuf_package_name(file.package());
                let file_name = files
                    .contains(file.name())
                    .then(|| module.to_file_name_or("_"));

                ((module.clone(), file.clone()), (module, file_name))
            })
            .unzip::<_, _, Vec<_>, HashMap<_, _>>();

        let mut generated_modules = prost_build::Config::new().generate(requests)?;

        if self.generate_serde {
            let mut json = pbjson_build::Builder::new();
            for file in &proto_file {
                json.register_file_descriptor(file.clone());
            }

            let results = json.generate(&["."], |package| {
                let module = module_from_package!(package);

                let content = generated_modules.remove(&module).unwrap_or_default();
                let mut cursor = io::Cursor::new(content.into_bytes());
                io::Seek::seek(&mut cursor, io::SeekFrom::End(0))?;
                Ok(cursor)
            })?;

            for (package, content) in results {
                generated_modules.insert(
                    module_from_package!(package),
                    String::from_utf8(content.into_inner())
                        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
                );
            }
        }

        let ret_val = generated_modules
            .into_iter()
            .filter_map(|(module, content)| {
                file_maps.remove(&module).and_then(|name| {
                    name.map(|name| code_generator_response::File {
                        name: Some(name),
                        content: Some(content),
                        ..code_generator_response::File::default()
                    })
                })
            })
            .collect();

        Ok(ret_val)
    }
}

impl str::FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret_val = Self::default();
        let params = s.split(',');
        for param in params {
            let trimmed = param.trim();
            if trimmed.is_empty() {
                continue;
            }

            let (key, value) = trimmed
                .split_once('=')
                .map(|(k, v)| (k, Some(v)))
                .unwrap_or((trimmed, None));
            match (key, value) {
                ("serde", _) => ret_val.generate_serde = true,
                _ => return Err(InvalidParameter(trimmed.to_string())),
            }
        }

        Ok(ret_val)
    }
}

#[derive(Debug)]
pub struct InvalidParameter(String);

impl fmt::Display for InvalidParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid parameter: ")?;
        f.write_str(&self.0)
    }
}

impl std::error::Error for InvalidParameter {}

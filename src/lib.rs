use prost::Message;
use prost_build::Module;
use prost_types::compiler::code_generator_response;
use prost_types::{FileDescriptorProto, FileDescriptorSet};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::{fmt, io, str};

#[derive(Debug, Default)]
pub struct Parameters {
    btree_map: Vec<String>,
    bytes: Vec<String>,
    disable_comments: Vec<String>,
    default_package_filename: Option<String>,
    extern_path: Vec<(String, String)>,
    type_attribute: Vec<(String, String)>,
    field_attribute: Vec<(String, String)>,
    compile_well_known_types: bool,
    retain_enum_prefix: bool,
    // file_descriptor_set: Option<String>,
    // include_file: Option<String>,
    // gen_crate: Option<String>,
}

struct FileGenerationParameters {}

macro_rules! module_from_package {
    ($package: ident) => {
        Module::from_parts($package.path().into_iter().map(|t| t.to_string()))
    };
}

impl Parameters {
    fn to_prost_config(&self) -> prost_build::Config {
        let mut config = prost_build::Config::new();
        config.btree_map(self.btree_map.iter());
        config.bytes(self.bytes.iter());
        config.disable_comments(self.disable_comments.iter());

        if let Some(filename) = self.default_package_filename.as_deref() {
            config.default_package_filename(filename);
        }

        for (proto_path, rust_path) in &self.extern_path {
            config.extern_path(proto_path, rust_path);
        }
        for (proto_path, attribute) in &self.type_attribute {
            config.type_attribute(proto_path, attribute);
        }
        for (proto_path, attribute) in &self.field_attribute {
            config.field_attribute(proto_path, attribute);
        }

        if self.compile_well_known_types {
            config.compile_well_known_types();
        }
        if self.retain_enum_prefix {
            config.retain_enum_prefix();
        }

        config
    }

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

        let mut generated_modules = self.to_prost_config().generate(requests)?;

        // if self.generate_serde {
        //     let mut json = pbjson_build::Builder::new();
        //     for file in &proto_file {
        //         json.register_file_descriptor(file.clone());
        //     }
        //
        //     let results = json.generate(&["."], |package| {
        //         let module = module_from_package!(package);
        //
        //         let content = generated_modules.remove(&module).unwrap_or_default();
        //         let mut cursor = io::Cursor::new(content.into_bytes());
        //         io::Seek::seek(&mut cursor, io::SeekFrom::End(0))?;
        //         Ok(cursor)
        //     })?;
        //
        //     for (package, content) in results {
        //         generated_modules.insert(
        //             module_from_package!(package),
        //             String::from_utf8(content.into_inner())
        //                 .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?,
        //         );
        //     }
        // }

        let mut ret_val = generated_modules
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
            .collect::<Vec<_>>();

        // if let Some(filename) = &self.file_descriptor_set {
        //     let fds = FileDescriptorSet { file: proto_file }.encode_to_vec();
        //
        //     let mut out_file = String::new();
        //     out_file.push_str("pub const FILE_DESCRIPTOR_SET: &[u8] = &[\n");
        //
        //     let mut chunks = fds.chunks_exact(16);
        //     while let Some(chunck) = chunks.next() {
        //         write!(
        //             out_file,
        //             "    {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x}, {:#04x},\n",
        //             chunck[0],
        //             chunck[1],
        //             chunck[2],
        //             chunck[3],
        //             chunck[4],
        //             chunck[5],
        //             chunck[6],
        //             chunck[7],
        //             chunck[8],
        //             chunck[9],
        //             chunck[10],
        //             chunck[11],
        //             chunck[12],
        //             chunck[13],
        //             chunck[14],
        //             chunck[15],
        //         ).unwrap();
        //     }
        //
        //     if !chunks.remainder().is_empty() {
        //         out_file.push_str("    ");
        //         for byte in chunks.remainder() {
        //             write!(out_file, "{:#04x}, ", byte).unwrap();
        //         }
        //         let _ = out_file.pop();
        //         out_file.push('\n');
        //     }
        //
        //     out_file.push_str("];\n");
        //
        //     ret_val.push(code_generator_response::File {
        //         name: Some(filename.to_string()),
        //         content: Some(out_file),
        //         ..code_generator_response::File::default()
        //     });
        // }

        Ok(ret_val)
    }
}

impl str::FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret_val = Self::default();
        let re = regex::Regex::new(
            r"(?:(?P<param>[^,=]+)(?:=(?P<key>[^,=]+)(?:=(?P<value>(?:[^,=\\]|\\,|\\)+))?)?)",
        )
        .unwrap();

        // let params = s.split(',');
        for capture in re.captures_iter(s) {
            let param = capture
                .get(1)
                .expect("any captured group will at least have the param name")
                .as_str()
                .trim();
            match (
                param,
                capture.get(2).map(|m| m.as_str()),
                capture.get(3).map(|m| m.as_str()),
            ) {
                // ("serde", _) => ret_val.generate_serde = true,
                ("btree_map", Some(value), None) => ret_val.btree_map.push(value.to_string()),
                ("bytes", Some(value), None) => ret_val.bytes.push(value.to_string()),
                ("default_package_filename", value, None) => {
                    ret_val.default_package_filename = value.map(|s| s.to_string())
                }
                ("compile_well_known_types", Some("true") | None, None) => {
                    ret_val.compile_well_known_types = true
                }
                ("compile_well_known_types", Some("false"), None) => (),
                ("disable_comments", Some(value), None) => {
                    ret_val.disable_comments.push(value.to_string())
                }
                ("retain_enum_prefix", Some("true") | None, None) => {
                    ret_val.retain_enum_prefix = true
                }
                ("retain_enum_prefix", Some("false"), None) => (),
                ("extern_path", Some(prefix), Some(module)) => ret_val
                    .extern_path
                    .push((prefix.to_string(), module.to_string())),
                ("type_attribute", Some(prefix), Some(module)) => ret_val
                    .type_attribute
                    .push((prefix.to_string(), module.replace(r"\,", ","))),
                ("field_attribute", Some(prefix), Some(module)) => ret_val
                    .field_attribute
                    .push((prefix.to_string(), module.replace(r"\,", ","))),
                // ("file_descriptor_set", value, None) => {
                //     ret_val.file_descriptor_set =
                //         Some(value.unwrap_or("file_descriptor_set.rs").to_string())
                // }
                _ => {
                    return Err(InvalidParameter(
                        capture.get(0).unwrap().as_str().to_string(),
                    ))
                }
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

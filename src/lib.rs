#![doc = include_str!("../README.md")]

use self::generator::{CoreProstGenerator, FileDescriptorSetGenerator, IncludeFileGenerator};
use once_cell::sync::Lazy;
use prost::Message;
use prost_build::Module;
use prost_types::compiler::code_generator_response::File;
use prost_types::compiler::CodeGeneratorRequest;
use prost_types::FileDescriptorProto;
use std::collections::{BTreeMap, HashSet};
use std::{cmp, fmt, str};

mod generator;

pub use self::generator::{Error, Generator, GeneratorResultExt, Result};

/// Execute the core _Prost!_ generator from an encoded [`CodeGeneratorRequest`]
pub fn execute(raw_request: &[u8]) -> generator::Result {
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

/// A set of requests to generate code for a series of modules
pub struct ModuleRequestSet {
    requests: BTreeMap<OrderableModule, ModuleRequest>,
}

impl ModuleRequestSet {
    /// Construct a new module request set from an encoded [`CodeGeneratorRequest`]
    ///
    /// [`CodeGeneratorRequest`]: prost_types::compiler::CodeGeneratorRequest
    pub fn new<I>(
        input_protos: I,
        proto_file: Vec<FileDescriptorProto>,
        raw_request: &[u8],
        default_package_filename: &str,
    ) -> std::result::Result<Self, prost::DecodeError>
    where
        I: IntoIterator<Item = String>,
    {
        let raw_protos = RawProtos::decode(raw_request)?;

        Ok(Self::new_decoded(
            input_protos,
            proto_file,
            raw_protos,
            default_package_filename,
        ))
    }

    fn new_decoded<I>(
        input_protos: I,
        proto_file: Vec<FileDescriptorProto>,
        raw_protos: RawProtos,
        default_package_filename: &str,
    ) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let input_protos: HashSet<_> = input_protos.into_iter().collect();

        let requests = proto_file.into_iter().zip(raw_protos.proto_file).fold(
            BTreeMap::new(),
            |mut acc, (proto, raw)| {
                let module = Module::from_protobuf_package_name(proto.package());
                let proto_filename = proto.name();
                let entry = acc.entry(OrderableModule(module)).or_insert_with(|| {
                    let mut request = ModuleRequest::new(proto.package().to_owned());
                    if input_protos.contains(proto_filename) {
                        let filename = match proto.package() {
                            "" => default_package_filename.to_owned(),
                            package => format!("{package}.rs"),
                        };
                        request.with_output_filename(filename);
                    }
                    request
                });

                entry.push_file_descriptor_proto(proto, raw);
                acc
            },
        );

        Self { requests }
    }

    /// An ordered iterator of all requests
    pub fn requests(&self) -> impl Iterator<Item = (&Module, &ModuleRequest)> {
        self.requests.iter().map(|(k, v)| (&k.0, v))
    }

    /// Retrieve the request for the given module
    pub fn for_module(&self, module: &Module) -> Option<&ModuleRequest> {
        self.requests.get(&OrderableModule(module.clone()))
    }
}

/// A code generation request for a specific module
pub struct ModuleRequest {
    proto_package_name: String,
    output_filename: Option<String>,
    files: Vec<FileDescriptorProto>,
    raw: Vec<Vec<u8>>,
}

impl ModuleRequest {
    fn new(proto_package_name: String) -> Self {
        Self {
            proto_package_name,
            output_filename: None,
            files: Vec::new(),
            raw: Vec::new(),
        }
    }

    fn with_output_filename(&mut self, filename: String) {
        self.output_filename = Some(filename);
    }

    fn push_file_descriptor_proto(&mut self, encoded: FileDescriptorProto, raw: Vec<u8>) {
        self.files.push(encoded);
        self.raw.push(raw);
    }

    /// The protobuf package name for this module
    pub fn proto_package_name(&self) -> &str {
        &self.proto_package_name
    }

    /// The output filename for this module
    pub fn output_filename(&self) -> Option<&str> {
        self.output_filename.as_deref()
    }

    /// An iterator of the file descriptors
    pub fn files(&self) -> impl Iterator<Item = &FileDescriptorProto> {
        self.files.iter()
    }

    /// An iterator of the encoded [`FileDescriptorProto`]s from [`files()`][Self::files()]
    pub fn raw_files(&self) -> impl Iterator<Item = &[u8]> {
        self.raw.iter().map(|b| b.as_slice())
    }

    /// Creates a code generation file from the output
    pub(crate) fn write_to_file<F: FnOnce(&mut String)>(&self, f: F) -> Option<File> {
        self.output_filename.as_deref().map(|name| {
            let mut content = String::new();
            f(&mut content);

            File {
                name: Some(name.to_owned()),
                content: Some(content),
                ..Default::default()
            }
        })
    }

    /// Appends generated code to the end of the main file for this module
    ///
    /// This is generally a good way to add includes referencing the output
    /// of other plugins or to directly append to the main file.
    pub fn append_to_file<F: FnOnce(&mut String)>(&self, f: F) -> Option<File> {
        self.output_filename.as_deref().map(|name| {
            let mut content = String::new();
            f(&mut content);

            File {
                name: Some(name.to_owned()),
                content: Some(content),
                insertion_point: Some("module".to_owned()),
                ..Default::default()
            }
        })
    }
}

/// Parameters use to configure [`Generator`]s built into `protoc-gen-prost`
///
/// [`Generator`]: crate::Generator
#[derive(Debug, Default)]
pub struct Parameters {
    /// Prost parameters, used to generate [`prost_build::Config`]
    pub prost: ProstParameters,

    /// Whether a file descriptor set has been requested in each module
    pub file_descriptor_set: bool,

    /// Whether to generate an include file with an optional filename
    pub include_file: Option<Option<String>>,
}

/// Parameters used to configure the underlying Prost generator
#[derive(Debug, Default)]
pub struct ProstParameters {
    btree_map: Vec<String>,
    bytes: Vec<String>,
    disable_comments: Vec<String>,
    default_package_filename: Option<String>,
    extern_path: Vec<(String, String)>,
    type_attribute: Vec<(String, String)>,
    field_attribute: Vec<(String, String)>,
    compile_well_known_types: bool,
    retain_enum_prefix: bool,
}

impl ProstParameters {
    /// Builds a [`prost_build::Config`] from the parameters
    pub fn to_prost_config(&self) -> prost_build::Config {
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

    pub fn default_package_filename(&self) -> &str {
        self.default_package_filename.as_deref().unwrap_or("_")
    }

    fn try_handle_parameter(
        &mut self,
        param: &str,
        key: Option<&str>,
        value: Option<&str>,
    ) -> std::result::Result<(), ()> {
        match (param, key, value) {
            ("btree_map", Some(value), None) => self.btree_map.push(value.to_string()),
            ("bytes", Some(value), None) => self.bytes.push(value.to_string()),
            ("default_package_filename", value, None) => {
                self.default_package_filename = value.map(|s| s.to_string())
            }
            ("compile_well_known_types", Some("true") | None, None) => {
                self.compile_well_known_types = true
            }
            ("compile_well_known_types", Some("false"), None) => (),
            ("disable_comments", Some(value), None) => {
                self.disable_comments.push(value.to_string())
            }
            ("retain_enum_prefix", Some("true") | None, None) => self.retain_enum_prefix = true,
            ("retain_enum_prefix", Some("false"), None) => (),
            ("extern_path", Some(prefix), Some(module)) => self
                .extern_path
                .push((prefix.to_string(), module.to_string())),
            ("type_attribute", Some(prefix), Some(module)) => self
                .type_attribute
                .push((prefix.to_string(), module.replace(r"\,", ","))),
            ("field_attribute", Some(prefix), Some(module)) => self
                .field_attribute
                .push((prefix.to_string(), module.replace(r"\,", ","))),
            _ => return Err(()),
        }

        Ok(())
    }
}

static PARAMETER: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(
        r"(?:(?P<param>[^,=]+)(?:=(?P<key>[^,=]+)(?:=(?P<value>(?:[^,=\\]|\\,|\\)+))?)?)",
    )
    .unwrap()
});

impl str::FromStr for Parameters {
    type Err = InvalidParameter;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut ret_val = Self::default();
        for capture in PARAMETER.captures_iter(s) {
            let param = capture
                .get(1)
                .expect("any captured group will at least have the param name")
                .as_str()
                .trim();

            let key = capture.get(2).map(|m| m.as_str());
            let value = capture.get(3).map(|m| m.as_str());

            if ret_val
                .prost
                .try_handle_parameter(param, key, value)
                .is_err()
            {
                match (param, key, value) {
                    // ("serde", _) => ret_val.generate_serde = true,
                    ("file_descriptor_set", Some("true") | None, None) => {
                        ret_val.file_descriptor_set = true
                    }
                    ("file_descriptor_set", Some("false"), None) => (),
                    ("include_file", filename, None) => {
                        ret_val.include_file = Some(filename.map(|s| s.to_owned()))
                    }
                    _ => {
                        return Err(InvalidParameter(
                            capture.get(0).unwrap().as_str().to_string(),
                        ))
                    }
                }
            }
        }

        Ok(ret_val)
    }
}

/// An invalid parameter
#[derive(Debug)]
pub struct InvalidParameter(String);

impl fmt::Display for InvalidParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid parameter: ")?;
        f.write_str(&self.0)
    }
}

impl std::error::Error for InvalidParameter {}

/// A wire-compatible reader of a [`CodeGeneratorRequest`]
///
/// This type treats the proto files contained in the request as raw byte
/// arrays so that we can round-trip those bytes into the generated files
/// as an encoded [`FileDescriptorSet`].
///
/// [`CodeGeneratorRequest`]: prost_types::compiler::CodeGeneratorRequest
/// [`FileDescriptorSet`]: prost_types::FileDescriptorSet
#[derive(Clone, PartialEq, ::prost::Message)]
struct RawProtos {
    #[prost(bytes = "vec", repeated, tag = "15")]
    proto_file: Vec<Vec<u8>>,
}

#[derive(PartialEq, Eq)]
struct OrderableModule(Module);

impl Ord for OrderableModule {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let mut my_path = self.0.parts();
        let mut other_path = other.0.parts();

        loop {
            let (my_part, other_part) = (my_path.next(), other_path.next());
            match (my_part, other_part) {
                (Some(my_part), Some(other_part)) => match my_part.cmp(other_part) {
                    cmp::Ordering::Equal => (),
                    ord => return ord,
                },
                (Some(_), None) => return cmp::Ordering::Greater,
                (None, Some(_)) => return cmp::Ordering::Less,
                (None, None) => return cmp::Ordering::Equal,
            }
        }
    }
}

impl PartialOrd for OrderableModule {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl fmt::Debug for OrderableModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for OrderableModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts = self.0.parts();
        if let Some(first) = parts.next() {
            f.write_str(first)?;
        }
        for part in parts {
            f.write_str("::")?;
            f.write_str(part)?;
        }
        Ok(())
    }
}

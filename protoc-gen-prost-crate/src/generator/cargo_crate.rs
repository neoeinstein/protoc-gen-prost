

use prost_types::compiler::code_generator_response::File;

use protoc_gen_prost::{Generator, ModuleRequest, ModuleRequestSet, Result};
use std::collections::{BTreeMap, BTreeSet};

pub struct CargoCrateGenerator {
    manifest_template: String,
}

impl CargoCrateGenerator {
    pub fn new(manifest_template: String) -> Self {
        Self { manifest_template }
    }
}

impl Generator for CargoCrateGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let result = module_request_set
            .requests()
            .map(|(_, v)| v)
            .collect::<PackageDependencies>();

        let mut buf = String::new();

        if self.manifest_template.contains("{{ features }}") {
            buf.push_str("default = []\n");
            if !result.dependency_graph.is_empty() {
                buf.push_str("full = [");
                for feature in result.dependency_graph.keys() {
                    buf.push('"');
                    buf.push_str(&feature.replace('.', "_"));
                    buf.push_str("\",");
                }
                let _ = buf.pop();
                buf.push_str("]\n");

                for (feature, dependencies) in result.dependency_graph {
                    buf.push_str(&feature.replace('.', "_"));
                    if dependencies.is_empty() {
                        buf.push_str(" = []\n");
                    } else {
                        buf.push_str(" = [");
                        for feature in dependencies {
                            buf.push('"');
                            buf.push_str(&feature.replace('.', "_"));
                            buf.push_str("\",");
                        }
                        let _ = buf.pop();
                        buf.push_str("]\n");
                    }
                }
            }
        }

        let content = self.manifest_template.replacen("{{ features }}", &buf, 1);

        let file = File {
            name: Some("Cargo.toml".to_string()),
            content: Some(content),
            ..File::default()
        };

        Ok(vec![file])
    }
}

struct PackageDependencies<'a> {
    pub dependency_graph: BTreeMap<&'a str, BTreeSet<&'a str>>,
}

impl<'a> FromIterator<&'a ModuleRequest> for PackageDependencies<'a> {
    fn from_iter<T: IntoIterator<Item = &'a ModuleRequest>>(iter: T) -> Self {
        let requests = iter.into_iter().collect::<Vec<&ModuleRequest>>();
        let mut features = requests
            .iter()
            .filter(|r| r.output_filename().is_some() && !r.proto_package_name().is_empty())
            .map(|r| (r.proto_package_name(), BTreeSet::new()))
            .collect::<BTreeMap<&str, BTreeSet<&str>>>();

        let mut depend_on_type = |current_package: &'a str, depends_on_type: &str| {
            // Only deal with fully-qualified paths
            if depends_on_type.starts_with('.') {
                // Search in reverse, relying on the fact that, for a given prefix, the more
                // specific package names will be after the higher-level
                // And don't make the package depend on itself
                if let Some(&key) = features
                    .keys()
                    .rev()
                    .filter(|&&key| key != current_package)
                    .find(|&&key| depends_on_type[1..].starts_with(&format!("{key}.")))
                {
                    features.entry(current_package).or_default().insert(key);
                }
            }
        };

        for file in requests.into_iter().flat_map(|r| r.files()) {
            for message in &file.message_type {
                for field in &message.field {
                    depend_on_type(file.package(), field.type_name());
                }
            }

            for service in &file.service {
                for method in &service.method {
                    depend_on_type(file.package(), method.input_type());
                    depend_on_type(file.package(), method.output_type());
                }
            }
        }

        Self {
            dependency_graph: features,
        }
    }
}

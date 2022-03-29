use prost_types::compiler::code_generator_response::File;

use crate::PackageLimiter;
use protoc_gen_prost::{Generator, ModuleRequest, ModuleRequestSet, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

pub(crate) struct FeaturesGenerator {
    include_filename: String,
    limiter: Rc<PackageLimiter>,
}

impl FeaturesGenerator {
    pub(crate) fn new(include_filename: String, limiter: Rc<PackageLimiter>) -> Self {
        Self {
            include_filename,
            limiter,
        }
    }
}

impl Generator for FeaturesGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let mut files = Vec::new();

        let dep_tree = module_request_set
            .requests()
            .map(|(_, v)| v)
            .collect::<PackageDependencies>();

        let mut buf = String::new();

        if !dep_tree.dependency_graph.is_empty() {
            buf.push_str("full = [");
            for feature in dep_tree.dependency_graph.keys() {
                if !self.limiter.is_allowed(feature) {
                    continue;
                }

                let feature_name = feature.replace('.', "_");
                buf.push('"');
                buf.push_str(&feature_name);
                buf.push_str("\",");

                files.push(File {
                    name: Some(self.include_filename.to_owned()),
                    content: Some(format!("#[cfg(feature = \"{}\")]\n", feature_name)),
                    insertion_point: Some(format!("attribute:{}", feature)),
                    ..File::default()
                });
            }
            if let Some('[') = buf.pop() {
                buf.push('[');
            }
            buf.push_str("]\n");

            for (feature, dependencies) in dep_tree.dependency_graph {
                if !self.limiter.is_allowed(feature) {
                    continue;
                }

                buf.push_str(&feature.replace('.', "_"));
                if dependencies.is_empty() {
                    buf.push_str(" = []\n");
                } else {
                    buf.push_str(" = [");
                    for feature in dependencies {
                        if !self.limiter.is_allowed(feature) {
                            continue;
                        }

                        buf.push('"');
                        buf.push_str(&feature.replace('.', "_"));
                        buf.push_str("\",");
                    }
                    if let Some('[') = buf.pop() {
                        buf.push('[');
                    }
                    buf.push_str("]\n");
                }
            }
        }

        files.push(File {
            name: Some("Cargo.toml".to_string()),
            content: Some(buf),
            insertion_point: Some("features".to_string()),
            ..File::default()
        });

        Ok(files)
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

use once_cell::sync::Lazy;
use prost_build::Module;
use prost_types::compiler::code_generator_response::File;
use protoc_gen_prost::{Generator, ModuleRequestSet, Result};

pub struct IncludeFileGenerator {
    filename: Option<String>,
    insert_features: bool,
}

impl IncludeFileGenerator {
    pub fn new(filename: Option<String>, insert_features: bool) -> Self {
        Self {
            filename,
            insert_features,
        }
    }
}

impl Generator for IncludeFileGenerator {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let mut context = CodeGenContext::new(self.insert_features);
        let mut buf = String::new();

        let _: () = module_request_set
            .requests()
            .filter_map(|(module, request)| {
                let filename = request.output_filename()?;

                context.move_to(module, request.proto_package_name(), &mut buf);
                context.push_include(filename, &mut buf);
                context.push_insertion_point(request.proto_package_name(), &mut buf);

                Some(())
            })
            .collect();

        context.finish(&mut buf);

        let file = File {
            name: Some(self.filename.as_deref().unwrap_or("mod.rs").to_string()),
            content: Some(buf),
            ..File::default()
        };

        Ok(vec![file])
    }
}

static ROOT_MODULE: Lazy<Module> = Lazy::new(|| Module::from_parts([] as [String; 0]));

#[must_use]
struct CodeGenContext<'a> {
    last: &'a Module,
    last_prefix: usize,
    indent: String,
    insert_features: bool,
}

const INDENT: &str = "    ";

impl<'a> CodeGenContext<'a> {
    fn new(insert_features: bool) -> Self {
        Self {
            last: &*ROOT_MODULE,
            last_prefix: 0,
            indent: String::new(),
            insert_features,
        }
    }

    fn indent(&mut self) {
        self.indent.push_str(INDENT);
    }

    fn dedent(&mut self) {
        self.indent.truncate(
            self.indent
                .len()
                .checked_sub(INDENT.len())
                .expect("indent underflow"),
        );
    }

    fn push_indent(&self, buf: &mut String) {
        buf.push_str(&self.indent);
    }

    fn move_to(&mut self, next: &'a Module, package: &str, buf: &mut String) {
        let (down, prefix) = difference(self.last, next);

        for _ in 0..down {
            self.close_module(buf);
        }

        let take = next.len() - prefix - 1;

        for module_name in next.parts().skip(prefix).take(take) {
            self.open_module(module_name, buf);
        }

        self.push_feature(package, buf);
        self.open_module(next.parts().last().unwrap(), buf);

        self.last = next;
        self.last_prefix = prefix;
    }

    fn finish(mut self, buf: &mut String) {
        for _ in 0..=self.last_prefix {
            self.close_module(buf)
        }
    }

    fn push_feature(&self, package: &str, buf: &mut String) {
        if self.insert_features {
            self.push_indent(buf);
            buf.push_str("#[cfg(feature = \"");
            buf.push_str(&package.replace('.', "_"));
            buf.push_str("\")]\n");
        }
    }

    fn push_include(&self, filename: &str, buf: &mut String) {
        self.push_indent(buf);
        buf.push_str("include!(\"");
        buf.push_str(filename);
        buf.push_str("\");\n");
    }

    fn push_insertion_point(&self, package_name: &str, buf: &mut String) {
        self.push_indent(buf);
        buf.push_str("// @@protoc_insertion_point(");
        buf.push_str(package_name);
        buf.push_str(")\n");
    }

    fn close_module(&mut self, buf: &mut String) {
        self.dedent();
        self.push_indent(buf);
        buf.push_str("}\n");
    }

    fn open_module(&mut self, module_name: &str, buf: &mut String) {
        self.push_indent(buf);
        buf.push_str("pub mod ");
        buf.push_str(module_name);
        buf.push_str(" {\n");
        self.indent();
    }
}

fn difference(left: &Module, right: &Module) -> (usize, usize) {
    let mut left_parts = left.parts();
    let mut right_parts = right.parts();

    let mut prefix = 0;

    loop {
        match (left_parts.next(), right_parts.next()) {
            (Some(left), Some(right)) if left == right => prefix += 1,
            (Some(_), Some(_)) => return (left_parts.count() + 1, prefix),
            (Some(_), None) => return (left_parts.count() + 1, prefix),
            (None, Some(_)) => return (0, prefix),
            (None, None) => return (0, prefix),
        }
    }
}

use crate::PackageLimiter;
use once_cell::sync::Lazy;
use prost_build::Module;
use prost_types::compiler::code_generator_response::File;
use protoc_gen_prost::{Generator, ModuleRequestSet, Result};
use std::rc::Rc;

const DEFAULT_FILENAME: &str = "mod.rs";

pub(crate) struct IncludeFileGenerator<'a> {
    filename: &'a str,
    limiter: Rc<PackageLimiter>,
}

impl<'a> IncludeFileGenerator<'a> {
    pub(crate) fn new(filename: Option<&'a str>, limiter: Rc<PackageLimiter>) -> Self {
        Self {
            filename: filename.unwrap_or(DEFAULT_FILENAME),
            limiter,
        }
    }

    pub(crate) fn filename(&self) -> &str {
        self.filename
    }
}

impl<'a> Generator for IncludeFileGenerator<'a> {
    fn generate(&mut self, module_request_set: &ModuleRequestSet) -> Result {
        let mut context = CodeGenContext::new();

        let _: () = module_request_set
            .requests()
            .filter_map(|(module, request)| {
                let filename = request.output_filename()?;
                self.limiter
                    .is_allowed(request.proto_package_name())
                    .then(|| {
                        context.move_to(module, request.proto_package_name());
                        context.push_include(filename);
                        context.push_insertion_point(request.proto_package_name());
                    })
            })
            .collect();

        let content = context.finish();

        let file = File {
            name: Some(self.filename.to_string()),
            content: Some(content),
            ..File::default()
        };

        Ok(vec![file])
    }
}

static ROOT_MODULE: Lazy<Module> = Lazy::new(|| Module::from_parts([] as [String; 0]));

#[must_use]
#[derive(Debug)]
struct CodeGenContext<'a> {
    last: &'a Module,
    indent: String,
    buf: String,
}

const INDENT: &str = "    ";

impl<'a> CodeGenContext<'a> {
    fn new() -> Self {
        Self {
            last: &*ROOT_MODULE,
            indent: String::new(),
            buf: String::new(),
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

    fn push_indent(&mut self) {
        self.buf.push_str(&self.indent);
    }

    fn move_to(&mut self, next: &'a Module, package: &str) {
        let (down, prefix) = difference(self.last, next);

        for _ in 0..down {
            self.close_module();
        }

        let take = next.len() - prefix - 1;

        for module_name in next.parts().skip(prefix).take(take) {
            self.open_module(module_name);
        }

        self.push_attribute_insertion_point(package);
        self.open_module(next.parts().last().unwrap());

        self.last = next;
    }

    fn finish(mut self) -> String {
        while !self.indent.is_empty() {
            self.close_module()
        }
        self.buf
    }

    fn push_include(&mut self, filename: &str) {
        self.push_indent();
        self.buf.push_str("include!(\"");
        self.buf.push_str(filename);
        self.buf.push_str("\");\n");
    }

    fn push_insertion_point(&mut self, package_name: &str) {
        self.push_indent();
        self.buf.push_str("// @@protoc_insertion_point(");
        self.buf.push_str(package_name);
        self.buf.push_str(")\n");
    }

    fn push_attribute_insertion_point(&mut self, package_name: &str) {
        self.push_indent();
        self.buf.push_str("// @@protoc_insertion_point(attribute:");
        self.buf.push_str(package_name);
        self.buf.push_str(")\n");
    }

    fn close_module(&mut self) {
        self.dedent();
        self.push_indent();
        self.buf.push_str("}\n");
    }

    fn open_module(&mut self, module_name: &str) {
        self.push_indent();
        self.buf.push_str("pub mod ");
        self.buf.push_str(module_name);
        self.buf.push_str(" {\n");
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

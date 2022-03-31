include!("gen/mod.rs");

// Types within the same crate can be extended with your own impl blocks like so:
#[cfg(feature = "example")]
impl example::ExampleRequest {
    pub fn borrow_request_value(&self) -> &str {
        &self.example
    }
}

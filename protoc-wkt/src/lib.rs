pub mod google {
    pub mod protobuf {
        include!("gen/google.protobuf.rs");
        pub mod compiler {
            include!("gen/google.protobuf.compiler.rs");
        }
    }
}

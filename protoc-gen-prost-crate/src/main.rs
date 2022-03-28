use prost::Message;
use protoc_gen_prost::GeneratorResultExt;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    let response = protoc_gen_prost_crate::execute(buf.as_slice()).unwrap_codegen_response();

    buf.clear();
    response.encode(&mut buf).expect("error encoding response");
    io::stdout().write_all(&buf)?;

    Ok(())
}

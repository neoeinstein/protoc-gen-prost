use std::error::Error;
use std::process::{exit, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let status = Command::new("buf")
        .arg("generate")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();

    if !status.success() {
        exit(status.code().unwrap_or(-1))
    }

    Ok(())
}

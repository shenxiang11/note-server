use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../protos/interactive.proto");

    fs::create_dir_all("src/pb")?;

    let builder = tonic_build::configure();

    builder
        .out_dir("src/pb")
        .compile_protos(&["../protos/interactive.proto"], &["../protos"])?;

    Ok(())
}

use anyhow::Result;
use std::{env, path::PathBuf, fs::File};

fn main() -> Result<()> {
    let output_path = PathBuf::from(env::var("OUT_DIR")?).join("index.rs");
    let ugly = false;
    fbs_build::compile_fbs_generic(
        ugly,
        None,
        Box::new(&pahkat_types::FLATBUFFERS_INDEX[..]),
        Box::new(File::create(output_path)?),
    )?;
    Ok(())
}

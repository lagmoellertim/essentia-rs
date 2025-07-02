use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = Path::new(&std::env::var("OUT_DIR").unwrap()).join("algorithms");

    essentia_codegen::generate_code(&out_dir)?;

    Ok(())
}

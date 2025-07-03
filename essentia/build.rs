use std::{fs::remove_dir_all, path::Path};

fn main() -> std::io::Result<()> {
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping build.rs on docs.rs");
        return Ok(());
    }

    println!("cargo:rerun-if-changed=build.rs");

    let directory = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated");

    if directory.exists() {
        remove_dir_all(&directory)?;
    }

    println!("cargo:rerun-if-changed=generated");

    essentia_codegen::generate_code(&directory)?;

    Ok(())
}

use std::path::Path;

fn main() -> std::io::Result<()> {
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping build.rs on docs.rs");
        return Ok(());
    }

    println!("cargo:rerun-if-changed=build.rs");

    let directory = Path::new(&std::env::var("OUT_DIR").unwrap()).join("algorithms");

    essentia_codegen::generate_code(&directory)?;

    Ok(())
}

use anyhow::Result;
use essentia::{algorithms::mono_loader::MonoLoader, Essentia, GetVariantData};

fn main() -> Result<()> {
    println!("-> Initializing Essentia...");
    let essentia = Essentia::new();
    println!("-> Essentia initialized successfully.");

    println!("-> Configuring MonoLoader algorithm...");
    let mut mono_loader = essentia::create::<MonoLoader>(&essentia)
        .filename("test.mp3")?
        .configure()?;
    println!("-> MonoLoader configured successfully.");

    println!("-> Computing MonoLoader...");
    let result = mono_loader.compute()?;
    println!("-> MonoLoader computed successfully.");

    println!("-> Accessing MonoLoader outputs...");
    if let Ok(audio) = result.audio() {
        let audio_vec = audio.get();
        println!("   - Audio length: {}", audio_vec.len());
    }

    println!("-> Example finished successfully.");
    Ok(())
}

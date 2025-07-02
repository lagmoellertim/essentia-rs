fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("bridge/bridge.cpp")
        .file("bridge/algorithm_bridge/core.cpp")
        .file("bridge/algorithm_bridge/input_output.cpp")
        .file("bridge/algorithm_bridge/introspection.cpp")
        .file("bridge/parameter_map_bridge/parameter_map_bridge.cpp")
        .file("bridge/pool_bridge/pool_bridge.cpp")
        .file("bridge/variant_data/accessors.cpp")
        .file("bridge/variant_data/constructors.cpp")
        .file("bridge/variant_data/introspection.cpp")
        .file("bridge/common/type_mapping.cpp")
        .include("target") // Include target directory so essentia/bridge/bridge.h can be found
        .include("bridge") // Add bridge directory to include path
        .include(".") // Add project root to include path for bridge/bridge.h
        .include("/usr/local/include")
        .include("/opt/homebrew/include")
        .include("/opt/homebrew/include/eigen3")
        .include("/opt/homebrew/opt/ffmpeg@4/include")
        .flag("-std=c++17")
        .flag("-DESSENTIA_VERSION=2")
        .flag("-DNO_TENSORFLOW") // Disable TensorFlow features
        .compile("essentia-bridge");

    println!("cargo:rustc-link-lib=essentia");
    println!("cargo:rustc-link-lib=yaml");
    println!("cargo:rustc-link-lib=fftw3f");
    println!("cargo:rustc-link-lib=tag");
    println!("cargo:rustc-link-lib=samplerate");
    println!("cargo:rustc-link-lib=tensorflow");
    println!("cargo:rustc-link-lib=chromaprint");

    // Try linking to FFmpeg libraries at older deployment target
    println!("cargo:rustc-link-lib=avformat");
    println!("cargo:rustc-link-lib=avcodec");
    println!("cargo:rustc-link-lib=avutil");
    println!("cargo:rustc-link-lib=swresample");

    // Link to frameworks on macOS
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }

    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/ffmpeg@4/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");

    println!("cargo:rerun-if-changed=bridge");
    println!("cargo:rerun-if-changed=src/ffi.rs");
}

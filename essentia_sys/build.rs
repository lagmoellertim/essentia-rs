struct Library {
    pub name: String,
    pub pkg_config_name: String,
    pub link_name: Option<String>,
}

impl Library {
    fn new(name: &str, pkg_config_name: &str, link_name: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            pkg_config_name: pkg_config_name.to_string(),
            link_name: link_name.map(|name| name.to_string()),
        }
    }
}

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping build.rs on docs.rs");
        return;
    }

    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .file("bridge/bridge.cpp")
        .file("bridge/algorithm_bridge/core.cpp")
        .file("bridge/algorithm_bridge/input_output.cpp")
        .file("bridge/algorithm_bridge/introspection.cpp")
        .file("bridge/parameter_map_bridge/parameter_map_bridge.cpp")
        .file("bridge/pool_bridge/pool_bridge.cpp")
        .file("bridge/data_container/accessors.cpp")
        .file("bridge/data_container/constructors.cpp")
        .file("bridge/data_container/introspection.cpp")
        .file("bridge/common/type_mapping.cpp")
        .include(".");

    let mut libraries = vec![
        Library::new("essentia", "essentia", Some("essentia")),
        Library::new("eigen3", "eigen3", None),
        Library::new("yaml", "yaml-0.1", Some("yaml")),
        Library::new("fftw3f", "fftw3f", Some("fftw3f")),
        Library::new("taglib", "taglib", Some("tag")),
        Library::new("samplerate", "samplerate", Some("samplerate")),
        Library::new("chromaprint", "libchromaprint", Some("chromaprint")),
        Library::new("avformat", "libavformat", Some("avformat")),
        Library::new("swresample", "libswresample", Some("swresample")),
        Library::new("avcodec", "libavcodec", Some("avcodec")),
        Library::new("avutil", "libavutil", Some("avutil")),
    ];

    let use_tensorflow = match std::env::var("USE_TENSORFLOW") {
        Ok(val) => !matches!(val.to_lowercase().as_str(), "0" | "false" | "no" | "off"),
        Err(_) => true,
    };

    if use_tensorflow {
        libraries.push(Library::new("tensorflow", "tensorflow", Some("tensorflow")))
    } else {
        println!("cargo:warning=Skipping tensorflow support as USE_TENSORFLOW=0");
    }

    for library in libraries {
        let pkg_info = match pkg_config::probe_library(&library.pkg_config_name) {
            Ok(pkg_info) => pkg_info,
            Err(err) => match library.name.as_str() {
                "tensorflow" => {
                    println!("cargo:error=Failed to find tensorflow: {}", err);
                    println!(
                        "cargo:error=If you intend to use essentia without tensorflow, set USE_TENSORFLOW=0"
                    );
                    std::process::exit(1);
                }
                _ => {
                    println!(
                        "cargo:error=Failed to find required library '{}': {}",
                        library.pkg_config_name, err
                    );
                    println!(
                        "cargo:error=Please install the library or check your pkg-config setup"
                    );
                    std::process::exit(1);
                }
            },
        };

        println!("{:?}", pkg_info);

        for mut include_path in pkg_info.include_paths {
            if library.name == "eigen3" {
                include_path.push("eigen3");
            }

            build.include(include_path);
        }

        for link_path in &pkg_info.link_paths {
            println!(
                "cargo:rustc-link-search=native={}",
                link_path.to_string_lossy()
            );
        }

        if let Some(link_name) = &library.link_name {
            println!("cargo:rustc-link-lib={}", link_name);
        }
    }

    build.flag("-std=c++17").compile("essentia-bridge");

    println!("cargo:rerun-if-changed=bridge");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

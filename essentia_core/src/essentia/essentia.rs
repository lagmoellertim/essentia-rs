use std::{
    collections::HashSet,
    sync::{Arc, Mutex, Weak},
};

use essentia_sys::ffi;
use once_cell::sync::Lazy;

use crate::{
    algorithm::{Algorithm, Initialized},
    essentia::error::CreateAlgorithmError,
};

static GLOBAL_LIFECYCLE: Lazy<Mutex<Weak<EssentiaLifecycle>>> =
    Lazy::new(|| Mutex::new(Weak::new()));

static AVAILABLE_ALGORITHMS: Lazy<HashSet<String>> =
    Lazy::new(|| ffi::get_algorithm_names().into_iter().collect());

/// Manages the global lifecycle of the Essentia library.
/// 
/// This struct ensures that `init_essentia()` is called when the first instance
/// is created and `shutdown_essentia()` is called when the last instance is dropped.
struct EssentiaLifecycle {}

impl EssentiaLifecycle {
    fn new() -> Self {
        ffi::init_essentia();
        Self {}
    }
}

impl Drop for EssentiaLifecycle {
    fn drop(&mut self) {
        ffi::shutdown_essentia();
    }
}

/// The main entry point for the Essentia audio analysis library.
/// 
/// This struct provides access to all available algorithms and manages the underlying
/// C++ library lifecycle. Multiple instances can be created safely - they share the
/// same underlying resources and the library will be properly initialized and
/// shutdown automatically.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use essentia_core::Essentia;
/// 
/// let essentia = Essentia::new();
/// 
/// // List all available algorithms
/// for algorithm_name in essentia.available_algorithms() {
///     println!("Available: {}", algorithm_name);
/// }
/// 
/// // Create a specific algorithm
/// let algorithm = essentia.create_algorithm("RhythmExtractor2013")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Essentia {
    _lifecycle: Arc<EssentiaLifecycle>,
}

impl Default for Essentia {
    fn default() -> Self {
        Self::new()
    }
}

impl Essentia {
    /// Creates a new Essentia instance.
    /// 
    /// This manages the global lifecycle of the Essentia library. Multiple instances
    /// can be created safely - they will share the same underlying initialization.
    /// The library will be automatically shut down when the last instance is dropped.
    pub fn new() -> Self {
        let mut global_lifecycle = GLOBAL_LIFECYCLE
            .lock()
            .expect("Failed to acquire lifecycle lock");

        if let Some(existing_lifecycle) = global_lifecycle.upgrade() {
            return Self {
                _lifecycle: existing_lifecycle,
            };
        }

        let lifecycle = Arc::new(EssentiaLifecycle::new());
        *global_lifecycle = Arc::downgrade(&lifecycle);

        Self {
            _lifecycle: lifecycle,
        }
    }

    /// Returns an iterator over all available algorithm names.
    /// 
    /// This provides access to all algorithms that are compiled into the Essentia library
    /// and can be instantiated using `create_algorithm`.
    pub fn available_algorithms(&self) -> impl Iterator<Item = &str> {
        AVAILABLE_ALGORITHMS.iter().map(|s| s.as_str())
    }

    /// Creates a new algorithm instance by name.
    /// 
    /// # Arguments
    /// 
    /// * `algorithm_name` - The name of the algorithm to create. Must be one of the names
    ///   returned by `available_algorithms()`.
    /// 
    /// # Returns
    /// 
    /// Returns an `Algorithm` in the `Initialized` state, ready for parameter configuration.
    /// 
    /// # Errors
    /// 
    /// Returns `CreateAlgorithmError` if the algorithm name is not recognized or if
    /// algorithm creation fails for any other reason.
    pub fn create_algorithm<'a>(
        &'a self,
        algorithm_name: &str,
    ) -> Result<Algorithm<'a, Initialized>, CreateAlgorithmError> {
        if !AVAILABLE_ALGORITHMS.contains(algorithm_name) {
            return Err(CreateAlgorithmError::AlgorithmNotFound {
                name: algorithm_name.to_string(),
            });
        }

        let algorithm_bridge = ffi::create_algorithm_bridge(algorithm_name)?;

        Ok(Algorithm::new(algorithm_bridge))
    }
}

impl Clone for Essentia {
    fn clone(&self) -> Self {
        Self {
            _lifecycle: Arc::clone(&self._lifecycle),
        }
    }
}

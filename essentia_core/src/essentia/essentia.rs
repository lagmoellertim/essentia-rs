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

pub struct Essentia {
    _lifecycle: Arc<EssentiaLifecycle>,
}

impl Essentia {
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

    pub fn available_algorithms(&self) -> impl Iterator<Item = &str> {
        AVAILABLE_ALGORITHMS.iter().map(|s| s.as_str())
    }

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

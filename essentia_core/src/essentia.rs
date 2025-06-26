use crate::algorithm::{Algorithm, Initialized};
use crate::ffi;

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex, Weak};

static GLOBAL_LIFECYCLE: Lazy<Mutex<Weak<EssentiaLifecycle>>> =
    Lazy::new(|| Mutex::new(Weak::new()));

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
        let mut global_lifecycle = GLOBAL_LIFECYCLE.lock().unwrap();

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

    pub fn available_algorithms(&self) -> Vec<String> {
        ffi::get_algorithm_names()
    }

    pub fn create_algorithm<'a>(&'a self, algorithm_name: &str) -> Algorithm<'a, Initialized> {
        Algorithm::new(ffi::create_algorithm(algorithm_name))
    }
}

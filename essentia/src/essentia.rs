use essentia_core::Initialized;

use crate::algorithm::CreateAlgorithm;

pub struct Essentia {
    inner: essentia_core::Essentia,
}

impl Essentia {
    pub fn new() -> Self {
        Self {
            inner: essentia_core::Essentia::new(),
        }
    }

    pub fn available_algorithms(&self) -> impl Iterator<Item = &str> {
        self.inner.available_algorithms()
    }

    pub fn create_from_name(
        &self,
        algorithm_name: &str,
    ) -> Result<
        crate::algorithm::dynamic::Algorithm<Initialized>,
        essentia_core::CreateAlgorithmError,
    > {
        self.inner.create_algorithm(algorithm_name)
    }

    pub fn create<'a, T: CreateAlgorithm<'a>>(&'a self) -> T::Output {
        T::create(self)
    }
}

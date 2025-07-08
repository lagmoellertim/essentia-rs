use crate::algorithm::CreateAlgorithm;

pub struct Essentia {
    pub(crate) inner: essentia_core::Essentia,
}

impl Default for Essentia {
    fn default() -> Self {
        Self::new()
    }
}

impl Essentia {
    pub fn new() -> Self {
        Self {
            inner: essentia_core::Essentia::new(),
        }
    }

    pub fn create<'a, T: CreateAlgorithm<'a>>(&'a self) -> T {
        T::create(self)
    }
}

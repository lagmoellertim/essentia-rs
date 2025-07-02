pub use essentia_core::algorithm::{Configured, Initialized};

pub mod dynamic {
    pub use essentia_core::algorithm::*;
}

mod error;
pub use error::*;

use crate::Essentia;

pub trait CreateAlgorithm<'a> {
    type Output;
    fn create(essentia: &'a Essentia) -> Self::Output;
}

include!(concat!(env!("OUT_DIR"), "/algorithms/mod.rs"));

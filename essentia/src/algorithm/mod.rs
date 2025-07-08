pub use essentia_core::algorithm::{Configured, Initialized};

mod error;
pub use error::*;

use crate::Essentia;

pub trait CreateAlgorithm<'a> {
    fn create(essentia: &'a Essentia) -> Self;
}

include!(concat!(env!("OUT_DIR"), "/algorithms/mod.rs"));

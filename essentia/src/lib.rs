pub mod algorithm;
pub mod essentia;
pub use essentia_core::{data, parameter_map, pool};

pub use essentia_core::data::{
    ConversionError, DataContainer, DataType, InputOutputData, IntoDataContainer, ParameterData,
    PoolData, TryGetFromDataContainer, TryIntoDataContainer, phantom,
};

pub use algorithm::{Configured, Initialized};
pub use essentia::Essentia;

pub use pool::{Pool, PoolError};

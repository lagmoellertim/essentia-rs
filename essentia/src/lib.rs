pub mod algorithm;
pub mod essentia;
pub use essentia_core::{data, parameter_map, pool};

pub use data::{
    ConversionError, DataContainer, DataType, GetFromDataContainer, InputOutputData,
    IntoDataContainer, ParameterData, PoolData, TryGetFromDataContainer, TryIntoDataContainer,
    data_type,
};

pub use algorithm::{Configured, Initialized};
pub use essentia::Essentia;

pub use pool::{Pool, PoolError};

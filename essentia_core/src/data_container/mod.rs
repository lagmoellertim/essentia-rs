mod data_container;
pub mod data_type;
mod data_type_enum;
mod error;
mod get_trait;
mod into_trait;

pub use data_container::DataContainer;
pub use data_type_enum::DataType;
pub use error::ConversionError;
pub use get_trait::TryGetFromDataContainer;
pub use get_trait::TryTryGetFromDataContainer;
pub use into_trait::IntoDataContainer;
pub use into_trait::TryIntoDataContainer;

#[cfg(test)]
mod test;

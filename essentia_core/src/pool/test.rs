use crate::pool::{Pool, PoolError};
use crate::variant_data::get_variant_data::GetVariantData;
use crate::variant_data::variant;
#[test]
fn test_new_pool_is_empty() {
    let pool = Pool::new();
    assert!(pool.is_empty());
    assert_eq!(pool.len(), 0);
}

#[test]
fn test_set_and_get_float() {
    let mut pool = Pool::new();
    let key = "test_float";
    let value = 3.14f32;
    pool.set(key, value);

    assert!(pool.contains(key));
    assert_eq!(pool.len(), 1);

    let retrieved_variant = pool.get::<variant::Float>(key).unwrap();
    let retrieved_value: f32 = retrieved_variant.get();
    assert_eq!(retrieved_value, value);
}

#[test]
fn test_set_and_get_vector_string() {
    let mut pool = Pool::new();
    let key = "test_vec_string";
    let value = vec!["hello", "world"];
    pool.set(key, value.as_slice());

    let retrieved: Vec<String> = pool.get::<variant::VectorString>(key).unwrap().get();
    let expected: Vec<String> = value.iter().map(|s| s.to_string()).collect();
    assert_eq!(retrieved, expected);
}

#[test]
fn test_get_non_existent_key() {
    let pool = Pool::new();
    let result = pool.get::<variant::Float>("non_existent");
    assert!(matches!(result, Err(PoolError::KeyNotFound { .. })));
}

#[test]
fn test_get_type_mismatch() {
    let mut pool = Pool::new();
    pool.set("a_float", 1.0f32);
    let result = pool.get::<variant::String>("a_float");
    assert!(matches!(result, Err(PoolError::TypeMismatch { .. })));
}

#[test]
fn test_keys() {
    let mut pool = Pool::new();
    pool.set("key1", 1.0f32);
    pool.set("key2", 2.0f32);
    let value = vec!["three"];
    pool.set("key3", value.as_slice());

    let mut keys = pool.keys();
    keys.sort();
    assert_eq!(keys, vec!["key1", "key2", "key3"]);
}

#[test]
fn test_overwrite_value() {
    let mut pool = Pool::new();
    let key = "value";
    pool.set(key, 10.0f32);
    let first_retrieved: f32 = pool.get::<variant::Float>(key).unwrap().get();
    assert_eq!(first_retrieved, 10.0);

    pool.set(key, 20.0f32);
    let second_retrieved: f32 = pool.get::<variant::Float>(key).unwrap().get();
    assert_eq!(second_retrieved, 20.0);
}

use essentia_sys::ffi;
use ndarray::{Array2, Array4};
use num::Complex;
use std::collections::HashMap;

use crate::data_container::ConversionError;

#[test]
fn test_bool() {
    let value = true;
    let data_container = value.into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_string() {
    let value = "hello world".to_string();
    let data_container = (&value).into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_i32() {
    let value = 42;
    let data_container = value.into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_f32() {
    let value = 3.14f32;
    let data_container = value.into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_u32() {
    let value = 123u32;
    let data_container = value.into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_i64() {
    let value = -123456789i64;
    let data_container = value.into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_stereo_sample() {
    let value = ffi::StereoSample {
        left: 0.1,
        right: -0.1,
    };
    let data_container = value.clone().into_data_container();
    let got = data_container.get();
    assert_eq!(got.left, value.left);
    assert_eq!(got.right, value.right);
}

#[test]
fn test_complex() {
    let value = Complex::new(1.0, -1.0);
    let data_container = value.into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_tensor_float() {
    let value = Array4::<f32>::zeros((1, 2, 3, 4));
    let data_container = (&value).into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_vector_bool() {
    let value = vec![true, false, true];
    let data_container = value.as_slice().into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_vector_int() {
    let value = vec![1, 2, 3];
    let data_container = value.as_slice().into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_vector_string() {
    let value = vec!["a", "b"];
    let data_container = value.as_slice().into_data_container();
    let expected: Vec<String> = value.iter().map(|s| s.to_string()).collect();
    assert_eq!(data_container.get(), expected);
}

#[test]
fn test_vector_float() {
    let value = vec![1.1, 2.2, 3.3f32];
    let data_container = value.as_slice().into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_vector_stereo_sample() {
    let value = vec![
        ffi::StereoSample {
            left: 0.1,
            right: -0.1,
        },
        ffi::StereoSample {
            left: 0.2,
            right: -0.2,
        },
    ];
    let data_container = value.as_slice().into_data_container();
    let got = data_container.get();
    assert_eq!(got.len(), value.len());
    for (g, v) in got.iter().zip(value.iter()) {
        assert_eq!(g.left, v.left);
        assert_eq!(g.right, v.right);
    }
}

#[test]
fn test_vector_complex() {
    let value = vec![Complex::new(1.0, -1.0), Complex::new(2.0, -2.0)];
    let data_container = value.as_slice().into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_matrix_float() {
    let value = Array2::from_shape_vec((2, 3), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0f32]).unwrap();
    let data_container = (&value).into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_vector_matrix_float() {
    let matrix1 = Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0f32]).unwrap();
    let matrix2 = Array2::from_shape_vec((2, 2), vec![5.0, 6.0, 7.0, 8.0f32]).unwrap();
    let value = vec![matrix1, matrix2];
    let data_container = value.as_slice().into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_vector_vector_float() {
    let value = vec![vec![1.1, 2.2], vec![3.3, 4.4f32]];
    let data_container = value.as_slice().into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_vector_vector_float_to_matrix() {
    let value = vec![vec![1.1, 2.2], vec![3.3, 4.4f32]];
    let data_container = value.as_slice().into_data_container();
    let matrix: Result<Array2<f32>, _> = data_container.try_get();
    let expected = Array2::from_shape_vec((2, 2), vec![1.1, 2.2, 3.3, 4.4f32]).unwrap();
    assert_eq!(matrix.unwrap(), expected);
}

#[test]
fn test_vector_vector_string() {
    let value = vec![vec!["a", "b"], vec!["c", "d"]];
    let slices: Vec<&[&str]> = value.iter().map(|v| v.as_slice()).collect();
    let data_container = slices.as_slice().into_data_container();
    let expected: Vec<Vec<String>> = value
        .iter()
        .map(|inner| inner.iter().map(|s| s.to_string()).collect())
        .collect();
    assert_eq!(data_container.get(), expected);
}

#[test]
fn test_vector_vector_stereo_sample() {
    let inner1 = vec![
        ffi::StereoSample {
            left: 0.1,
            right: -0.1,
        },
        ffi::StereoSample {
            left: 0.2,
            right: -0.2,
        },
    ];
    let inner2 = vec![
        ffi::StereoSample {
            left: 0.3,
            right: -0.3,
        },
        ffi::StereoSample {
            left: 0.4,
            right: -0.4,
        },
    ];
    let value = vec![inner1.as_slice(), inner2.as_slice()];
    let data_container = value.as_slice().into_data_container();
    let got = data_container.get();
    assert_eq!(got.len(), value.len());
    for (g_vec, v_vec) in got.iter().zip(value.iter()) {
        assert_eq!(g_vec.len(), v_vec.len());
        for (g, v) in g_vec.iter().zip(v_vec.iter()) {
            assert_eq!(g.left, v.left);
            assert_eq!(g.right, v.right);
        }
    }
}

#[test]
fn test_vector_vector_complex() {
    let value = vec![
        vec![Complex::new(1.0, -1.0), Complex::new(2.0, -2.0)],
        vec![Complex::new(3.0, -3.0), Complex::new(4.0, -4.0)],
    ];
    let data_container = value.as_slice().into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_map_float() {
    let mut value = HashMap::new();
    value.insert("a".to_string(), 1.1f32);
    value.insert("b".to_string(), 2.2f32);
    let data_container = (&value).into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_map_vector_float() {
    let mut value = HashMap::new();
    value.insert("a".to_string(), vec![1.1, 2.2f32]);
    value.insert("b".to_string(), vec![3.3, 4.4f32]);
    let data_container = (&value).into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_map_vector_string() {
    let mut value = HashMap::new();
    value.insert("a".to_string(), vec!["s1".to_string(), "s2".to_string()]);
    value.insert("b".to_string(), vec!["s3".to_string(), "s4".to_string()]);
    let data_container = (&value).into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_map_vector_int() {
    let mut value = HashMap::new();
    value.insert("a".to_string(), vec![1, 2]);
    value.insert("b".to_string(), vec![3, 4]);
    let data_container = (&value).into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_map_vector_complex() {
    let mut value = HashMap::new();
    value.insert(
        "a".to_string(),
        vec![Complex::new(1.0, -1.0), Complex::new(2.0, -2.0)],
    );
    value.insert(
        "b".to_string(),
        vec![Complex::new(3.0, -3.0), Complex::new(4.0, -4.0)],
    );
    let data_container = (&value).into_data_container();
    assert_eq!(data_container.get(), value);
}

#[test]
fn test_pool() {
    let mut pool = Pool::new();
    let float_vec = vec![1.0, 2.0, 3.0f32];
    pool.set("some_float", 3.14f32);
    pool.set("some_vec", float_vec.as_slice());

    let data_container = pool.into_data_container();
    let retrieved_pool: Pool = data_container.get();

    let f: f32 = retrieved_pool.get("some_float").unwrap().get();
    assert_eq!(f, 3.14f32);

    let v: Vec<f32> = retrieved_pool.get("some_vec").unwrap().get();
    assert_eq!(v, float_vec);
}

#[test]
fn test_vector_vector_float_to_matrix_empty() {
    let value: Vec<Vec<f32>> = vec![];
    let data_container = value.as_slice().into_data_container();
    let matrix: Result<Array2<f32>, ConversionError> = data_container.try_get();
    assert!(matches!(matrix, Err(ConversionError::EmptyMatrix)));
}

#[test]
fn test_vector_vector_float_to_matrix_empty_rows() {
    let value: Vec<Vec<f32>> = vec![vec![]];
    let data_container = value.as_slice().into_data_container();
    let matrix: Result<Array2<f32>, ConversionError> = data_container.try_get();
    assert!(matches!(matrix, Err(ConversionError::EmptyRows)));
}

#[test]
fn test_vector_vector_float_to_matrix_non_rectangular() {
    let value = vec![vec![1.0, 2.0], vec![3.0]];
    let data_container = value.as_slice().into_data_container();
    let matrix: Result<Array2<f32>, ConversionError> = data_container.try_get();
    assert!(matches!(
        matrix,
        Err(ConversionError::NonRectangular { .. })
    ));
}

#[allow(dead_code)]

use pyo3::{prelude::*, PyDowncastError};
use pyo3::types::{PyDict,PyList,PyUnicode,PyTuple};
use sprs::{CsMat, CsMatViewI};
use numpy::{PyReadonlyArrayDyn};

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

#[pyfunction]
fn string_identity(x: &PyUnicode) -> &PyUnicode {
    x
}

#[pyfunction]
fn handle_list(l: &PyList) -> usize {
    l.len()
}

#[pyfunction]
fn debug_print(p: &PyAny) -> Result<(), PyErr> {


    let shape:&PyTuple = p.getattr("shape")?.extract()?;
    let shape_tuple = (shape.get_item(0)?.extract::<usize>()?, shape.get_item(1)?.extract::<usize>()?);
    println!("shape={:?}",shape_tuple);

    let indptr:PyReadonlyArrayDyn<i32> = p.getattr("indptr")?.extract()?;
    let indptr_array = indptr.as_slice()?;
    println!("indptr_array={:?}",indptr_array);

    let data:PyReadonlyArrayDyn<f64> = p.getattr("data")?.extract()?;
    let data_array = data.as_slice()?;
    println!("data_array={:?}",data_array);

    let indices:PyReadonlyArrayDyn<i32> = p.getattr("indices")?.extract()?;
    println!("indices={:?}", indices);
    let indices_array = indices.as_slice()?;
    println!("indices_array={:?}", indices_array);

    let sprs_mat = CsMatViewI::new_csc(
        shape_tuple,
        indptr_array,
        indices_array,
        data_array,
    );
    println!("tada: {:?}", sprs_mat);
    return Ok(());
    // println!("{:?}", p.;
}

/// This module is implemented in Rust.
#[pymodule]
#[pyo3(name = "sparse_matrix_rust_to_python")]
fn my_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;
    m.add_function(wrap_pyfunction!(string_identity, m)?)?;
    m.add_function(wrap_pyfunction!(handle_list, m)?)?;
    m.add_function(wrap_pyfunction!(debug_print, m)?)?;
    Ok(())
}

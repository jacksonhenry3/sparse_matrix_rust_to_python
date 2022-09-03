use pyo3::{prelude::*, PyDowncastError};
use pyo3::types::{PyDict,PyList,PyUnicode};
use sprs::{CsMat};



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


    let shape = p.getattr("shape")?;
    let indptr = p.getattr("indptr")?;
    let data = p.getattr("data")?;
    let indices = p.getattr("indices")?;
    println!("indptr={:?},\ndata={:?},\nindices={:?},\nshape={:?}", indptr,data,indices, shape);
    // any.downcast::<SpecificPyType>()?;
    // let sprs_mat = CsMat::new_csc(shape,
    //                    indptr,
    //                    indices,
    //                    data);

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

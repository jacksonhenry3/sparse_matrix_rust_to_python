use pyo3::prelude::*;

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

/// This module is implemented in Rust.
#[pymodule]
#[pyo3(name = "sparse_matrix_rust_to_python")]
fn my_extension(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;
    Ok(())
}

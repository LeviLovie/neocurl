use pyo3::{prelude::*, types::PyType};

#[pyclass(name = "Response")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PyResponse {
    #[pyo3(get)]
    pub status: String,

    #[pyo3(get)]
    pub status_code: u16,

    #[pyo3(get)]
    pub headers: Option<Vec<(String, String)>>,

    #[pyo3(get)]
    pub body: Option<String>,

    #[pyo3(get)]
    pub elapsed: u64,
}

#[pymethods]
impl PyResponse {
    #[classmethod]
    fn elapsed_seconds(_cls: &Bound<'_, PyType>, elapsed: u64) -> f64 {
        elapsed as f64 / 1_000_000.0 // Convert nanoseconds to seconds
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyResponse>()?;

    Ok(())
}

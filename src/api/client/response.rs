use pyo3::prelude::*;

#[pyclass(name = "Response")]
#[derive(Debug, Clone, PartialEq)]
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

    #[pyo3(get)]
    pub elapsed_seconds: f64,
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyResponse>()?;

    Ok(())
}

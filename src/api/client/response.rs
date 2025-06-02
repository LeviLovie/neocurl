use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass(name = "Response")]
#[derive(Debug, Clone, PartialEq)]
pub struct PyResponse {
    #[pyo3(get)]
    pub status: String,

    #[pyo3(get)]
    pub status_code: u16,

    #[pyo3(get)]
    pub headers: HashMap<String, String>,

    #[pyo3(get)]
    pub body: String,

    #[pyo3(get)]
    pub body_raw: Vec<u8>,

    #[pyo3(get)]
    pub duration: u64,
}

#[pymethods]
impl PyResponse {
    fn print(&self) {
        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("({}: {})", k, v))
            .collect::<Vec<String>>()
            .join(",\n    ");

        println!("Response:");
        println!("  Status: {} {}", self.status_code, self.status);
        println!("  Duration: {:}", self.duration);
        println!("  Headers:\n    {}", headers);
        println!("  Body:\n{}", self.body);
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyResponse>()?;

    Ok(())
}

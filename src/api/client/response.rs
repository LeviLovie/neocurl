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
        println!("{}", self.dump());
    }

    fn dump(&self) -> String {
        let mut dump = String::new();

        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("({}: {})", k, v))
            .collect::<Vec<String>>()
            .join(",\n    ");

        dump.push_str("Response:\n");
        dump.push_str(&format!("  Status: {} {}\n", self.status_code, self.status));
        dump.push_str(&format!("  Duration: {:}\n", self.duration));
        dump.push_str(&format!("  Headers:\n    {}\n", headers));
        dump.push_str(&format!("  Body:\n{}\n", self.body));

        dump
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyResponse>()?;

    Ok(())
}

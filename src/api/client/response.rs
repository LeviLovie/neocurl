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

#[pymethods]
impl PyResponse {
    fn print(&self) {
        let headers = self.headers.as_ref().map_or("None".to_string(), |h| {
            h.iter()
                .map(|(k, v)| format!("({}: {})", k, v))
                .collect::<Vec<_>>()
                .join(",\n    ")
        });

        println!("Response:");
        println!("  Status: {} {}", self.status_code, self.status);
        println!("  Elapsed: {:.2}s", self.elapsed_seconds);
        println!("  Headers:\n    {}", headers);
        println!(
            "  Body:\n{}",
            self.body.as_ref().map_or("None".to_string(), |b| b.clone())
        );
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyResponse>()?;

    Ok(())
}

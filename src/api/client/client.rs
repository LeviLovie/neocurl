use super::{PyRequest, PyResponse};
use pyo3::{prelude::*, types::PyType};

#[pyclass(name = "Client")]
pub struct PyClient {}

#[pymethods]
impl PyClient {
    #[new]
    fn __new__() -> Self {
        PyClient {}
    }

    #[classmethod]
    fn send(_cls: &Bound<'_, PyType>, request: PyRequest) -> PyResult<PyResponse> {
        let request_builder = request.to_reqwest_blocking();

        let start = std::time::Instant::now();
        let response = request_builder.send().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Request failed: {}", e))
        })?;
        let duration = start.elapsed();

        let status_code = response.status().as_u16();
        let status = response.status().to_string();

        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect::<Vec<_>>();

        let response_body = response.text().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to read response body: {}",
                e
            ))
        })?;

        Ok(PyResponse {
            status_code,
            status,
            headers: Some(headers),
            body: Some(response_body),
            elapsed: duration.as_nanos() as u64,
        })
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyClient>()?;

    Ok(())
}

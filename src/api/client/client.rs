use super::{PyMethod, PyRequest, PyResponse};
use pyo3::{prelude::*, types::PyDict};

#[pyclass(name = "Client")]
pub struct PyClient {}

impl PyClient {
    fn send_request(&self, request: PyRequest) -> PyResult<PyResponse> {
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
            elapsed_seconds: duration.as_secs_f64(),
        })
    }
}

#[pymethods]
impl PyClient {
    #[new]
    fn __new__() -> Self {
        PyClient {}
    }

    #[pyo3(signature = (url, **kwargs))]
    fn send(&mut self, url: String, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<PyResponse> {
        let method = kwargs
            .and_then(|d| d.get_item("method").ok()?)
            .and_then(|m| m.extract::<PyMethod>().ok())
            .unwrap_or(PyMethod::Get);

        let request = PyRequest::from_kwargs(url, method, kwargs)?;
        self.send_request(request)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn get(&mut self, url: String, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<PyResponse> {
        let request = PyRequest::from_kwargs(url, PyMethod::Get, kwargs)?;
        self.send_request(request)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn post(&mut self, url: String, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<PyResponse> {
        let request = PyRequest::from_kwargs(url, PyMethod::Post, kwargs)?;
        self.send_request(request)
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyClient>()?;

    Ok(())
}

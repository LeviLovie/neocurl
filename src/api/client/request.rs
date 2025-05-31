use super::PyMethod;
use pyo3::prelude::*;

#[pyclass(name = "Request")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PyRequest {
    #[pyo3(get, set)]
    pub url: String,

    #[pyo3(get, set)]
    pub method: PyMethod,

    #[pyo3(get, set)]
    pub headers: Option<Vec<(String, String)>>,

    #[pyo3(get, set)]
    pub params: Option<Vec<(String, String)>>,

    #[pyo3(get, set)]
    pub body: Option<String>,

    #[pyo3(get, set)]
    pub timeout: u64,
}

#[pymethods]
impl PyRequest {
    #[new]
    fn __new__(url: String) -> Self {
        PyRequest {
            url,
            method: PyMethod::Get, // Default method is GET
            headers: None,
            params: None,
            body: None,
            timeout: 100_000, // Default timeout of 100 seconds
        }
    }
}

impl PyRequest {
    #[allow(dead_code)]
    pub fn to_reqwest(&self) -> reqwest::RequestBuilder {
        let client = reqwest::Client::new();

        let mut request_builder = match self.method {
            PyMethod::Get => client.get(&self.url),
            PyMethod::Post => client.post(&self.url),
        };

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
        }

        if let Some(params) = &self.params {
            for (key, value) in params {
                request_builder = request_builder.query(&[(key.as_str(), value.as_str())]);
            }
        }

        if let Some(body) = &self.body {
            request_builder = request_builder.body(body.clone());
        }

        request_builder.timeout(std::time::Duration::from_millis(self.timeout))
    }

    pub fn to_reqwest_blocking(&self) -> reqwest::blocking::RequestBuilder {
        let client = reqwest::blocking::Client::new();

        let mut request_builder = match self.method {
            PyMethod::Get => client.get(&self.url),
            PyMethod::Post => client.post(&self.url),
        };

        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
        }

        if let Some(params) = &self.params {
            for (key, value) in params {
                request_builder = request_builder.query(&[(key.as_str(), value.as_str())]);
            }
        }

        if let Some(body) = &self.body {
            request_builder = request_builder.body(body.clone());
        }

        request_builder.timeout(std::time::Duration::from_millis(self.timeout))
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyRequest>()?;

    Ok(())
}

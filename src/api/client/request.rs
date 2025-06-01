use super::PyMethod;
use pyo3::{prelude::*, types::PyDict};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PyRequest {
    pub url: String,
    pub method: PyMethod,
    pub headers: Option<Vec<(String, String)>>,
    pub params: Option<Vec<(String, String)>>,
    pub body: Option<Vec<u8>>,
    pub timeout: u64,
}

impl PyRequest {
    /// Creates a new PyRequest instance from a URL and optional keyword arguments.
    pub fn from_kwargs(
        url: String,
        method: PyMethod,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let body_py = kwargs.and_then(|d| d.get_item("body").ok()?);
        let body = if let Some(body_py) = body_py {
            match body_py.extract::<String>() {
                Ok(body) => Some(body.as_bytes().to_vec()),
                Err(_) => {
                    // Try to parse as bytes
                    match body_py.extract::<Vec<u8>>() {
                        Ok(bytes) => Some(bytes),
                        Err(_) => {
                            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                                "Body must be a string or bytes",
                            ))
                        }
                    }
                }
            }
        } else {
            None
        };

        let headers_py = kwargs.and_then(|d| d.get_item("headers").ok()?);
        let headers = if let Some(headers_py) = headers_py {
            match headers_py.extract::<Vec<(String, String)>>() {
                Ok(headers) => Some(headers),
                Err(_) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Headers must be a list of tuples (key, value)",
                    ))
                }
            }
        } else {
            None
        };

        let params_py = kwargs.and_then(|d| d.get_item("params").ok()?);
        let params = if let Some(params_py) = params_py {
            match params_py.extract::<Vec<(String, String)>>() {
                Ok(params) => Some(params),
                Err(_) => {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Params must be a list of tuples (key, value)",
                    ))
                }
            }
        } else {
            None
        };

        let timeout = kwargs
            .and_then(|d| d.get_item("timeout").ok()?)
            .and_then(|v| v.extract::<u64>().ok())
            .unwrap_or(100_000); // Default timeout of 100 seconds

        Ok(PyRequest {
            url,
            method,
            headers,
            params,
            body,
            timeout,
        })
    }

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

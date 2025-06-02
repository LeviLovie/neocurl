use super::{PyMethod, PyRequest, PyResponse};
use indicatif::{ProgressBar, ProgressStyle};
use pyo3::{prelude::*, types::PyDict};
use reqwest::Client;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{Semaphore, mpsc},
    task,
};

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

        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let response_body = response.text().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to read response body: {}",
                e
            ))
        })?;

        Ok(PyResponse {
            status_code,
            status,
            headers,
            body: Some(response_body),
            duration: duration.as_millis() as u64,
        })
    }

    fn send_requests_async(
        &self,
        request: PyRequest,
        amount: u32,
        threads: u32,
    ) -> PyResult<Vec<PyResponse>> {
        let progress_bar = ProgressBar::new(amount.into());
        let style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>5}/{len:5} {msg}",
        )
        .unwrap()
        .progress_chars("##-");
        progress_bar.set_style(style.clone());
        progress_bar.set_message("Processing");

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(threads as usize)
            .enable_all()
            .build()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Runtime error: {}", e))
            })?;

        let results = rt.block_on(async {
            let request_template = Arc::new(
                request
                    .to_reqwest()
                    .build()
                    .expect("Failed to build request"),
            );
            let semaphore = Arc::new(Semaphore::new(threads as usize));
            let (tx, mut rx) = mpsc::unbounded_channel();

            let sending_start = std::time::Instant::now();
            let per_thread = amount / threads;

            let mut handles = Vec::new();

            for _ in 0..threads {
                let tx = tx.clone();
                let semaphore = semaphore.clone();
                let request = request_template.clone();
                let progress_bar = progress_bar.clone();

                let handle = task::spawn(async move {
                    let client = Client::new(); // one client per thread

                    for _ in 0..per_thread {
                        let _permit = semaphore.acquire().await.unwrap();
                        let req = request.try_clone().expect("Failed to clone request");

                        let start = std::time::Instant::now();

                        if let Ok(response) = client.execute(req).await {
                            let duration = start.elapsed();
                            let status_code = response.status().as_u16();
                            let status = response.status().to_string();
                            let headers: HashMap<String, String> = response
                                .headers()
                                .iter()
                                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                                .collect();

                            let body = (response.text().await).ok();

                            let response = PyResponse {
                                status_code,
                                status,
                                headers,
                                body,
                                duration: duration.as_millis() as u64,
                            };

                            if let Err(e) = tx.send(response) {
                                eprintln!("Failed to send response: {}", e);
                            }
                        }

                        progress_bar.inc(1);
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                handle.await.expect("Thread failed");
            }

            drop(tx);

            let mut responses = Vec::with_capacity(amount as usize);
            while let Some(res) = rx.recv().await {
                responses.push(res);
            }

            progress_bar.finish_and_clear();
            let duration = sending_start.elapsed();
            println!(
                "Done in {} ms, {:.2} req/s",
                duration.as_millis(),
                amount as f64 / (duration.as_secs_f64() + 1e-9)
            );

            responses
        });

        Ok(results)
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

        let request = PyRequest::from_args(url, method, kwargs)?;
        self.send_request(request)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn send_async(
        &mut self,
        url: String,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Vec<PyResponse>> {
        let method = kwargs
            .and_then(|d| d.get_item("method").ok()?)
            .and_then(|m| m.extract::<PyMethod>().ok())
            .unwrap_or(PyMethod::Get);

        let amount = kwargs
            .and_then(|d| d.get_item("amount").ok()?)
            .and_then(|v| v.extract::<u32>().ok())
            .unwrap_or(1);

        let threads = kwargs
            .and_then(|d| d.get_item("threads").ok()?)
            .and_then(|v| v.extract::<u32>().ok())
            .unwrap_or(1);

        let request = PyRequest::from_args(url, method, kwargs)?;
        self.send_requests_async(request, amount, threads)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn get(&mut self, url: String, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<PyResponse> {
        let request = PyRequest::from_args(url, PyMethod::Get, kwargs)?;
        self.send_request(request)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn get_async(
        &mut self,
        url: String,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Vec<PyResponse>> {
        let request = PyRequest::from_args(url, PyMethod::Get, kwargs)?;

        let amount = kwargs
            .and_then(|d| d.get_item("amount").ok()?)
            .and_then(|v| v.extract::<u32>().ok())
            .unwrap_or(1);

        let threads = kwargs
            .and_then(|d| d.get_item("threads").ok()?)
            .and_then(|v| v.extract::<u32>().ok())
            .unwrap_or(1);

        self.send_requests_async(request, amount, threads)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn post(&mut self, url: String, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<PyResponse> {
        let request = PyRequest::from_args(url, PyMethod::Post, kwargs)?;
        self.send_request(request)
    }

    #[pyo3(signature = (url, **kwargs))]
    fn post_async(
        &mut self,
        url: String,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Vec<PyResponse>> {
        let request = PyRequest::from_args(url, PyMethod::Post, kwargs)?;

        let amount = kwargs
            .and_then(|d| d.get_item("amount").ok()?)
            .and_then(|v| v.extract::<u32>().ok())
            .unwrap_or(1);
        let threads = kwargs
            .and_then(|d| d.get_item("threads").ok()?)
            .and_then(|v| v.extract::<u32>().ok())
            .unwrap_or(1);

        self.send_requests_async(request, amount, threads)
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyClient>()?;

    Ok(())
}

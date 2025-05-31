mod client;
mod define;
mod on_init;
mod version;

use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::sync::Mutex;

pub use client::PyClient;

pub static REGISTRY: Lazy<Mutex<Vec<Py<PyAny>>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ON_INIT: Lazy<Mutex<Option<Py<PyAny>>>> = Lazy::new(|| Mutex::new(None));

#[pymodule(name = "neocurl")]
pub fn neocurl_py_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    client::register(m)?;
    define::register(m)?;
    on_init::register(m)?;
    version::register(m)?;

    Ok(())
}

mod client;
mod define;
mod env;
mod logger;
mod on_cleanup;
mod on_init;
mod tests;
mod version;

pub use client::PyClient;
pub use logger::{LOGGER_CONFIG, PyLogLevel};

use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::sync::Mutex;

pub static REGISTRY: Lazy<Mutex<Vec<Py<PyAny>>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ON_INIT: Lazy<Mutex<Option<Py<PyAny>>>> = Lazy::new(|| Mutex::new(None));
pub static ON_CLEANUP: Lazy<Mutex<Option<Py<PyAny>>>> = Lazy::new(|| Mutex::new(None));
pub static TESTS: Lazy<Mutex<(u32, u32)>> = Lazy::new(|| Mutex::new((0, 0)));
pub static CALLS: Lazy<Mutex<(u32, u32)>> = Lazy::new(|| Mutex::new((0, 0)));

#[pymodule(name = "neocurl")]
pub fn neocurl_py_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    client::register(m)?;
    define::register(m)?;
    env::register(m)?;
    logger::register(m)?;
    on_cleanup::register(m)?;
    on_init::register(m)?;
    tests::register(m)?;
    version::register(m)?;

    Ok(())
}

mod define;
mod version;

use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::sync::Mutex;

pub struct Definition {
    pub name: String,
    pub func: Py<PyAny>,
}

pub static REGISTRY: Lazy<Mutex<Vec<Definition>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn make_rust_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let module = PyModule::new(py, "neocurl")?;

    version::register(&module)?;
    define::register(&module)?;

    Ok(module)
}

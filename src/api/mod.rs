mod define;
mod version;

use pyo3::prelude::*;

pub fn make_rust_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    let module = PyModule::new(py, "neocurl")?;

    version::register(&module)?;
    define::register(&module)?;

    Ok(module)
}

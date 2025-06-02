use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
fn version() -> PyResult<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

#[pyfunction]
fn check_version(requested: String) -> PyResult<bool> {
    let version = env!("CARGO_PKG_VERSION");

    Ok(version == requested)
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(version, module)?)?;
    module.add_function(wrap_pyfunction!(check_version, module)?)?;

    Ok(())
}

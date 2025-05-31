use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
fn env(var: String) -> PyResult<Option<String>> {
    if dotenv::dotenv().is_ok() {
        Ok(dotenv::var(var).ok())
    } else {
        Ok(std::env::var(var).ok())
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(env, module)?)?;

    Ok(())
}

use pyo3::prelude::*;

#[pyclass(eq, name = "Method")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PyMethod {
    Get,
    Post,
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyMethod>()?;

    Ok(())
}

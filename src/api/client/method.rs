use pyo3::prelude::*;

#[pyclass(eq, name = "Method")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PyMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Patch,
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyMethod>()?;
    module.add("GET", PyMethod::Get)?;
    module.add("HEAD", PyMethod::Head)?;
    module.add("POST", PyMethod::Post)?;
    module.add("PUT", PyMethod::Put)?;
    module.add("DELETE", PyMethod::Delete)?;
    module.add("PATCH", PyMethod::Patch)?;

    Ok(())
}

use super::REGISTRY;
use pyo3::prelude::*;

#[pyclass(name = "define")]
pub struct PyDefine {}

#[pymethods]
impl PyDefine {
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self {
        let mut registry = REGISTRY.lock().unwrap();
        registry.push(wraps);

        PyDefine {}
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyDefine>()?;

    Ok(())
}

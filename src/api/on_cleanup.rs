use super::ON_CLEANUP;
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};

#[pyclass(name = "on_cleanup")]
pub struct PyOnCleanup {}

#[pymethods]
impl PyOnCleanup {
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self {
        ON_CLEANUP.lock().unwrap().replace(wraps);

        PyOnCleanup {}
    }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        let on_init = ON_CLEANUP.lock().unwrap();
        let on_init_ref = on_init.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("on_init function not set")
        })?;
        let ret = on_init_ref.call(py, args, kwargs)?;

        Ok(ret)
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyOnCleanup>()?;

    Ok(())
}

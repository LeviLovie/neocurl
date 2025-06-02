use super::ON_INIT;
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};

#[pyclass(name = "on_init")]
pub struct PyOnInit {}

#[pymethods]
impl PyOnInit {
    #[new]
    fn __new__(wraps: Py<PyAny>) -> Self {
        ON_INIT.lock().unwrap().replace(wraps);

        PyOnInit {}
    }

    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        let on_init = ON_INIT.lock().unwrap();
        let on_init_ref = on_init.as_ref().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("on_init function not set")
        })?;
        let ret = on_init_ref.call(py, args, kwargs)?;

        Ok(ret)
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyOnInit>()?;

    Ok(())
}

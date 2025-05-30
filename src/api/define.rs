use super::{Definition, REGISTRY};
use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
fn define(name: String, func: PyObject) -> PyResult<()> {
    let mut registry = REGISTRY.lock().unwrap();
    registry.push(Definition {
        name,
        func: func.into(),
    });

    Ok(())
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(define, module)?)?;

    Ok(())
}

use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
fn define() -> PyResult<()> {
    // println!("Received data: {:?}", data);
    Ok(())
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(define, module)?)?;

    Ok(())
}

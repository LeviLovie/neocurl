use super::TESTS;
use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
fn assert_t(cond: bool) -> PyResult<bool> {
    let mut tests = TESTS.lock().unwrap();
    let pass = cond;

    if pass {
        (*tests).0 += 1;
    } else {
        (*tests).1 += 1;
    }

    return Ok(pass);
}

#[pyfunction]
fn assert_f(cond: bool) -> PyResult<bool> {
    let mut tests = TESTS.lock().unwrap();
    let pass = !cond;

    if pass {
        (*tests).0 += 1;
    } else {
        (*tests).1 += 1;
    }

    return Ok(pass);
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(assert_t, module)?)?;
    module.add_function(wrap_pyfunction!(assert_f, module)?)?;

    Ok(())
}

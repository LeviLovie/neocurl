mod client_py;
mod method;
mod request;
mod response;

pub use client_py::PyClient;
pub use method::PyMethod;
pub use request::PyRequest;
pub use response::PyResponse;

use pyo3::prelude::*;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    client_py::register(module)?;
    method::register(module)?;
    response::register(module)?;

    Ok(())
}

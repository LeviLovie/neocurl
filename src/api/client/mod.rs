mod client;
mod method;
mod request;
mod response;

pub use client::PyClient;
pub use method::PyMethod;
pub use request::PyRequest;
pub use response::PyResponse;

use pyo3::prelude::*;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    client::register(module)?;
    method::register(module)?;
    request::register(module)?;
    response::register(module)?;

    Ok(())
}

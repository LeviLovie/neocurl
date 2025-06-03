mod client;
mod define;
mod env;
mod logger;
mod on_cleanup;
mod on_init;
mod tests;
mod version;

pub use client::PyClient;
pub use logger::{LOGGER_CONFIG, PyLogLevel};

use pyo3::prelude::*;

#[pymodule(name = "neocurl")]
pub fn neocurl_py_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    client::register(m)?;
    define::register(m)?;
    env::register(m)?;
    logger::register(m)?;
    on_cleanup::register(m)?;
    on_init::register(m)?;
    tests::register(m)?;
    version::register(m)?;

    Ok(())
}

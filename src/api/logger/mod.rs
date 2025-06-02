mod level;
mod log;
mod logger_config;

pub use level::PyLogLevel;
pub use logger_config::PyLoggerConfig;

use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::sync::Mutex;

pub static LOGGER_CONFIG: Lazy<Mutex<PyLoggerConfig>> =
    Lazy::new(|| Mutex::new(PyLoggerConfig::default()));

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    level::register(module)?;
    log::register(module)?;
    logger_config::register(module)?;

    Ok(())
}

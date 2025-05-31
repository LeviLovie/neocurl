use super::{PyLogLevel, LOGGER_CONFIG};
use pyo3::prelude::*;

#[pyclass(name = "LoggerConfig")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PyLoggerConfig {
    #[pyo3(get, set)]
    pub use_colors: bool,

    #[pyo3(get, set)]
    pub level: PyLogLevel,

    #[pyo3(get, set)]
    pub datetime_format: String,

    pub context: Option<String>,
}

impl Default for PyLoggerConfig {
    fn default() -> Self {
        PyLoggerConfig {
            use_colors: true,
            level: PyLogLevel::Info,
            datetime_format: "%Y-%m-%d %H:%M:%S".to_string(),
            context: None,
        }
    }
}

impl PyLoggerConfig {
    pub fn set_context(&mut self, context: String) {
        self.context = Some(context);
    }

    pub fn clear_context(&mut self) {
        self.context = None;
    }
}

#[pyfunction]
fn get_logger_config() -> PyResult<PyLoggerConfig> {
    let config = LOGGER_CONFIG.lock().unwrap().clone();

    Ok(config)
}

#[pyfunction]
fn set_logger_config(config: PyLoggerConfig) -> PyResult<()> {
    *LOGGER_CONFIG.lock().unwrap() = config;

    Ok(())
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyLoggerConfig>()?;
    module.add_function(wrap_pyfunction!(get_logger_config, module)?)?;
    module.add_function(wrap_pyfunction!(set_logger_config, module)?)?;

    Ok(())
}

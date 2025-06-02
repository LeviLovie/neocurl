use owo_colors::OwoColorize;
use pyo3::prelude::*;

#[pyclass(eq, name = "LogLevel")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PyLogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl PyLogLevel {
    pub fn format(&self, use_color: bool) -> String {
        if use_color {
            match self {
                PyLogLevel::Debug => format!("{}", "DEBUG".blue()),
                PyLogLevel::Info => format!("{}", "INFO".green()),
                PyLogLevel::Warn => format!("{}", "WARN".yellow()),
                PyLogLevel::Error => format!("{}", "ERROR".red()),
                PyLogLevel::Fatal => format!("{}", "FATAL".bright_red()),
            }
        } else {
            match self {
                PyLogLevel::Debug => "DEBUG".to_string(),
                PyLogLevel::Info => "INFO".to_string(),
                PyLogLevel::Warn => "WARN".to_string(),
                PyLogLevel::Error => "ERROR".to_string(),
                PyLogLevel::Fatal => "FATAL".to_string(),
            }
        }
    }

    pub fn less_than(&self, other: &PyLogLevel) -> bool {
        match (self, other) {
            (PyLogLevel::Debug, _) => true,
            (PyLogLevel::Info, PyLogLevel::Debug) => false,
            (PyLogLevel::Info, _) => true,
            (PyLogLevel::Warn, PyLogLevel::Debug | PyLogLevel::Info) => false,
            (PyLogLevel::Warn, _) => true,
            (PyLogLevel::Error, PyLogLevel::Debug | PyLogLevel::Info | PyLogLevel::Warn) => false,
            (PyLogLevel::Error, _) => true,
            (PyLogLevel::Fatal, _) => false,
        }
    }
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyLogLevel>()?;

    Ok(())
}

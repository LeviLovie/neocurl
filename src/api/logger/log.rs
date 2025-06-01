use super::{LOGGER_CONFIG, PyLogLevel, PyLoggerConfig};
use owo_colors::{OwoColorize, XtermColors};
use pyo3::prelude::*;

fn format_log(config: PyLoggerConfig, level: PyLogLevel, msg: &str) -> PyResult<String> {
    let timestamp = chrono::Utc::now().format(&config.datetime_format);
    let context = match config.context {
        Some(c) => format!(" {}", c),
        None => "".to_string(),
    };

    if config.use_colors {
        Ok(format!(
            "{} {}{}{} {}",
            timestamp.color(XtermColors::DarkGray),
            level.format(config.use_colors),
            context.color(XtermColors::DarkGray),
            ":".color(XtermColors::DarkGray),
            msg
        ))
    } else {
        Ok(format!(
            "{} {}{}: {}",
            timestamp,
            level.format(config.use_colors),
            context,
            msg
        ))
    }
}

#[pyfunction]
fn log(level: PyLogLevel, msg: String) -> PyResult<()> {
    let config = LOGGER_CONFIG.lock().unwrap().clone();

    if !config.level.less_than(&level) {
        return Ok(());
    }

    println!("{}", format_log(config, level.clone(), &msg)?);

    if level == PyLogLevel::Fatal {
        std::process::exit(1);
    }

    Ok(())
}

#[pyfunction]
fn debug(msg: String) -> PyResult<()> {
    log(PyLogLevel::Debug, msg)
}

#[pyfunction]
fn info(msg: String) -> PyResult<()> {
    log(PyLogLevel::Info, msg)
}

#[pyfunction]
fn warn(msg: String) -> PyResult<()> {
    log(PyLogLevel::Warn, msg)
}

#[pyfunction]
fn error(msg: String) -> PyResult<()> {
    log(PyLogLevel::Error, msg)
}

#[pyfunction]
fn fatal(msg: String) -> PyResult<()> {
    log(PyLogLevel::Fatal, msg)
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(log, module)?)?;
    module.add_function(wrap_pyfunction!(debug, module)?)?;
    module.add_function(wrap_pyfunction!(info, module)?)?;
    module.add_function(wrap_pyfunction!(warn, module)?)?;
    module.add_function(wrap_pyfunction!(error, module)?)?;
    module.add_function(wrap_pyfunction!(fatal, module)?)?;

    Ok(())
}

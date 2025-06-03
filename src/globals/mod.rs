use once_cell::sync::Lazy;
use pyo3::prelude::*;
use std::sync::Mutex;

pub static REGISTRY: Lazy<Mutex<Vec<Py<PyAny>>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static ON_INIT: Lazy<Mutex<Option<Py<PyAny>>>> = Lazy::new(|| Mutex::new(None));
pub static ON_CLEANUP: Lazy<Mutex<Option<Py<PyAny>>>> = Lazy::new(|| Mutex::new(None));
pub static TESTS: Lazy<Mutex<(u32, u32)>> = Lazy::new(|| Mutex::new((0, 0)));
pub static CALLS: Lazy<Mutex<(u32, u32)>> = Lazy::new(|| Mutex::new((0, 0)));

use anyhow::{Context, Result};
use pyo3::{ffi::c_str, prelude::*, types::PyAnyMethods, Python};
use std::{ffi::CString, path::PathBuf};

pub struct VmBuilder {
    loaded: Option<(PathBuf, String)>,
}

impl VmBuilder {
    pub fn new() -> Self {
        VmBuilder { loaded: None }
    }

    /// Adds a source code file to the VM builder.
    #[tracing::instrument(skip_all, fields(source))]
    pub fn load(mut self, source: String) -> Result<Self> {
        if self.loaded.is_some() {
            tracing::warn!("Overwriting previously loaded source code file");
        }

        let content = read_file(source).context("Failed to read source file")?;
        let source = content.trim().to_string();

        if !source.is_empty() {
            let path = PathBuf::from(source.clone());
            self.loaded = Some((path, source));
        } else {
            return Err(anyhow::anyhow!("Source code file is empty"));
        }

        Ok(self)
    }

    pub fn build(self) -> Result<Vm> {
        if self.loaded.is_none() {
            return Err(anyhow::anyhow!("No source code file loaded"));
        }
        let (path, source) = self.loaded.unwrap();

        return Ok(Vm {
            source,
            _path: path,
        });
    }
}

pub struct Vm {
    source: String,
    _path: PathBuf,
}

impl Vm {
    pub fn builder() -> VmBuilder {
        VmBuilder::new()
    }

    pub fn run(&self) -> Result<()> {
        Python::with_gil(|py| {
            load_venv_libs(py).context("Failed to load virtual environment libraries")?;
            add_neocurl_module(py).context("Failed to add neocurl module")?;

            let code = CString::new(self.source.clone())
                .expect("Failed to create CString from source code");
            let _module = PyModule::from_code(py, &code, c_str!("neocurl.py"), c_str!("neocurl"))
                .context("Failed to create module from code")?;

            // let global = module.getattr("on_init").context("Failed to get on_init")?;
            // let _ = global.call0()?;

            Ok(())
        })
    }

    pub fn run_definition(&self, name: String) -> Result<()> {
        Python::with_gil(|py| {
            for def in crate::api::REGISTRY.lock().unwrap().iter() {
                if def.name == name {
                    let func = def.func.as_ref();
                    func.call0(py)
                        .context(format!("Failed to call definition: {}", name))?;
                    return Ok(());
                }
            }
            Ok(())
        })
    }
}

/// Reads the file specified in the arguments
fn read_file(file: String) -> Result<String> {
    let file_path = std::path::Path::new(&file);

    if !file_path.exists() {
        return Err(anyhow::anyhow!("File does not exist: {}", file));
    }

    let file_contents =
        std::fs::read_to_string(file_path).context(format!("Failed to read file: {}", file))?;

    Ok(file_contents)
}

/// Try to find venv
fn use_venv() -> Option<String> {
    if std::env::var("VIRTUAL_ENV").is_ok() {
        if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
            return Some(venv);
        }
    }

    None
}

/// Load libs from venv
fn load_venv_libs(py: Python<'_>) -> Result<()> {
    let sys = py.import("sys")?;
    let version: String = sys.getattr("version")?.extract()?;
    tracing::debug!("Python version: {}", version);

    if let Some(venv) = use_venv() {
        let site_packages = PathBuf::from(venv)
            .join("lib")
            .join("python3.11")
            .join("site-packages");
        let site = py.import("site")?;
        site.call_method1("addsitedir", (site_packages,))?;
    } else {
        tracing::warn!(
            "No virtual environment found, using system Python: {}",
            version
        );
    }

    Ok(())
}

/// Add neocurl module
fn add_neocurl_module(py: Python<'_>) -> Result<()> {
    let neocurl = crate::api::make_rust_module(py)?;
    let sys = py.import("sys")?;
    sys.getattr("modules")?
        .set_item("neocurl", neocurl)
        .context("Failed to set neocurl module in sys.modules")?;

    Ok(())
}

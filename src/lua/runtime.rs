use tracing::error;

use super::{api, libs, RequestRegistry};

/// Builder for LuaRuntime
pub struct LuaRuntimeBuilder {
    script: Option<String>,
    modules: Vec<(String, String)>,
}

impl LuaRuntimeBuilder {
    /// Create a new LuaRuntimeBuilder
    pub fn new() -> Self {
        Self {
            script: None,
            modules: Vec::new(),
        }
    }

    /// Set the script to be executed
    pub fn with_script(mut self, script: String) -> Self {
        self.script = Some(script);
        self
    }

    /// Add a module
    #[allow(dead_code)]
    pub fn add_module(mut self, name: &str, script: &str) -> Self {
        self.modules.push((name.to_string(), script.to_string()));
        self
    }

    /// Add a library
    fn add_lib(&mut self, (name, script): (&str, &str)) {
        self.modules.push((name.to_string(), script.to_string()));
    }

    /// Add default libraries
    pub fn libs(mut self) -> Self {
        self.add_lib(libs::LIB_DKJSON);
        self
    }

    /// Build the LuaRuntime
    pub fn build(self) -> anyhow::Result<LuaRuntime> {
        let span = tracing::info_span!("build");
        let _enter = span.enter();

        if self.script.is_none() {
            error!("Script is required");
            return Err(anyhow::anyhow!("Script is required"));
        }
        let script = self.script.unwrap();

        let registry: RequestRegistry = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        let lua = mlua::Lua::new();
        api::reg(&lua, registry.clone(), script.clone())?;

        for (name, script) in self.modules {
            let register = format!(
                r#"
package = package or {{}}
package.preload = package.preload or {{}}
package.preload["{}"] = function(...)
    {}
end"#,
                name, script
            );
            lua.load(register).exec()?;
        }

        lua.load(&script).exec()?;

        Ok(LuaRuntime {
            _lua: lua,
            registry,
        })
    }
}

/// Abstraction to execute Lua scripts
pub struct LuaRuntime {
    _lua: mlua::Lua,
    registry: RequestRegistry,
}

impl LuaRuntime {
    /// Create a builder
    pub fn builder() -> LuaRuntimeBuilder {
        LuaRuntimeBuilder::new()
    }

    /// Return a summary of all tests executed
    pub fn test_summary(&self) -> (usize, usize) {
        api::test_summary()
    }

    /// List all definitions in the registry
    pub fn list_refinitions(&self) -> Vec<String> {
        self.registry
            .lock()
            .unwrap()
            .iter()
            .map(|req| req.get::<String>("name").unwrap_or_default())
            .collect()
    }

    /// Run a definition in the registry by name
    pub fn run_definition(&mut self, name: String) -> anyhow::Result<()> {
        let span = tracing::info_span!("run_definition");
        let _enter = span.enter();

        run_definition_in_registry(self.registry.clone(), name)?;

        Ok(())
    }
}

/// Run a definition in the registry by name
pub fn run_definition_in_registry(registry: RequestRegistry, name: String) -> anyhow::Result<()> {
    let span = tracing::info_span!("run_definition_in_registry");
    let _enter = span.enter();

    let registry = registry.lock().unwrap().clone();
    let definition = registry
        .iter()
        .find(|req| req.get::<String>("name").unwrap_or_default() == name)
        .ok_or_else(|| anyhow::anyhow!("Definition {} can not be found", name))?;

    let request_str: mlua::Function = definition.get("func")?;
    let _: () = request_str.call(())?;

    Ok(())
}

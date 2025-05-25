use owo_colors::{OwoColorize, XtermColors};
use tracing::error;

use super::{RequestRegistry, api, libs};

/// Builder for LuaRuntime
pub struct LuaRuntimeBuilder {
    script: Option<String>,
    modules: Vec<(String, String)>,
    main_dir: Option<std::path::PathBuf>,
    thread_name: String,
}

impl LuaRuntimeBuilder {
    /// Create a new LuaRuntimeBuilder
    pub fn new() -> Self {
        Self {
            script: None,
            modules: Vec::new(),
            main_dir: None,
            thread_name: "main".to_string(),
        }
    }

    /// Set the main directory
    pub fn with_main_dir(mut self, main_dir: std::path::PathBuf) -> Self {
        self.main_dir = Some(main_dir);
        self
    }

    /// Set the script to be executed
    pub fn with_script(mut self, script: String) -> Self {
        self.script = Some(script);
        self
    }

    /// Set the thread name
    pub fn with_thread(mut self, thread_name: String) -> Self {
        self.thread_name = thread_name;
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

        if self.main_dir.is_none() {
            error!("Main directory is required");
            return Err(anyhow::anyhow!("Main directory is required"));
        }
        let main_dir = self.main_dir.unwrap();

        if self.script.is_none() {
            error!("Script is required");
            return Err(anyhow::anyhow!("Script is required"));
        }
        let script = self.script.unwrap();

        let registry: RequestRegistry = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

        let lua = mlua::Lua::new();
        api::reg(
            &lua,
            registry.clone(),
            script.clone(),
            main_dir,
            self.thread_name.clone(),
        )?;

        for (name, script) in self.modules {
            let register = format!(
                r#"
                    package = package or {{}}
                    package.preload = package.preload or {{}}
                    package.preload["{}"] = function(...)
                        {}
                    end
                "#,
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
    pub fn list_refinitions(&self) -> Vec<(bool, String)> {
        self.registry
            .lock()
            .unwrap()
            .iter()
            .map(|req| {
                let is_a_test = match req.get::<mlua::Value>("test").unwrap() {
                    mlua::Value::Nil => true,
                    mlua::Value::Boolean(b) => b,
                    _ => {
                        error!("Invalid type for 'foo' in request: {:?}", req);
                        false
                    }
                };
                (is_a_test, req.get::<String>("name").unwrap_or_default())
            })
            .collect()
    }

    /// Run a definition in the registry by name
    pub fn run_definition(&mut self, name: String) -> anyhow::Result<()> {
        let span = tracing::info_span!("run_definition");
        let _enter = span.enter();

        run_definition_in_registry(self.registry.clone(), name)?;

        Ok(())
    }

    /// Run all tests in the registry
    pub fn run_tests(&mut self) -> anyhow::Result<()> {
        let span = tracing::info_span!("run_tests");
        let _enter = span.enter();

        let mut global_passed = 0;
        let mut global_failed = 0;
        let mut failed_tests = Vec::new();

        for (is_a_test, name) in self.list_refinitions() {
            if !is_a_test {
                println!("Skipping non-test definition: {}", name);
                continue;
            }
            println!("Running test: {}", name);

            self.run_definition(name.clone())?;
            let (passed, failed) = self.test_summary();
            global_passed += passed;
            global_failed += failed;

            if failed > 0 {
                failed_tests.push(name);
            }

            println!();
        }

        println!(
            "{} {}{}{}",
            "Test overview:".color(XtermColors::DarkGray),
            global_passed.bright_green().bold(),
            "|".color(XtermColors::DarkGray),
            global_failed.bright_red().bold()
        );

        if global_failed > 0 {
            println!("Failed tests: {:#?}", failed_tests);
            std::process::exit(1);
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_script() {
        let script = r#"
            function test()
                return "Hello, World!"
            end
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }

    #[test]
    fn invalid_script() {
        let script = r#"
            function test()
                return "Hello, World!"
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .add_module("test", "return 1")
            .build();

        assert!(runtime.is_err());
    }

    #[test]
    fn run_definition_in_registry_success() {
        let script = r#"
            define({
                name = "test",
                func = function() end,
            })
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build()
            .unwrap();
        let name = "test".to_string();
        let result = run_definition_in_registry(runtime.registry.clone(), name);

        assert!(result.is_ok());
    }

    #[test]
    fn run_definition_in_registry_missing() {
        let script = r#"
            define({
                name = "test",
                func = function() end,
            })
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build()
            .unwrap();
        let name = "missing".to_string();
        let result = run_definition_in_registry(runtime.registry.clone(), name);

        assert!(result.is_err());
    }
}

use super::super::runtime;

pub fn reg(
    lua: &mlua::Lua,
    registry: crate::lua::RequestRegistry,
    file_contents: String,
    main_dir: std::path::PathBuf,
) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_run(lua, registry.clone())?;
    reg_run_async(lua, file_contents, main_dir.clone())?;

    Ok(())
}

fn reg_run(lua: &mlua::Lua, registry: crate::lua::RequestRegistry) -> anyhow::Result<()> {
    let span = tracing::debug_span!("reg_run");
    let _enter = span.enter();

    let globals = lua.globals();
    let run_fn = lua
        .create_function(move |_, (name, amount): (String, Option<u32>)| {
            let amount = if let Some(amount) = amount { amount } else { 1 };
            tracing::info!("Running request: {} ({})", name, amount);

            for _ in 0..amount {
                runtime::run_definition_in_registry(registry.clone(), name.clone()).map_err(
                    |e| {
                        tracing::error!("Failed to run request: {}", e);
                        mlua::prelude::LuaError::runtime("Failed to run request")
                    },
                )?;
            }

            Ok(())
        })
        .map_err(|e| {
            tracing::error!("Failed to create run function: {}", e);
            anyhow::anyhow!("Failed to create run function")
        })?;
    globals.set("run", run_fn).map_err(|e| {
        tracing::error!("Failed to set run function in globals: {}", e);
        anyhow::anyhow!("Failed to set run function in globals")
    })?;

    Ok(())
}

fn reg_run_async(lua: &mlua::Lua, file_contents: String, main_dir: std::path::PathBuf) -> anyhow::Result<()> {
    let span = tracing::debug_span!("reg_run_async");
    let _enter = span.enter();

    let globals = lua.globals();
    let run_async_fn = lua
        .create_function(
            move |_, (name, amount, delay): (mlua::Table, Option<u32>, Option<u64>)| {
                let amount = if let Some(amount) = amount { amount } else { 1 };
                let names = name
                    .pairs()
                    .filter_map(|pair| {
                        let (_, value): (String, String) = pair.ok()?;
                        Some(value)
                    })
                    .collect::<Vec<String>>();
                let delay = if let Some(delay) = delay {
                    std::time::Duration::from_millis(delay as u64)
                } else {
                    std::time::Duration::from_millis(100)
                };

                run_lua_tasks_async(file_contents.clone(), main_dir.clone(), names, amount, delay).map_err(|e| {
                    tracing::error!("Failed to run request: {}", e);
                    mlua::prelude::LuaError::runtime("Failed to run request")
                })?;

                Ok(())
            },
        )
        .map_err(|e| {
            tracing::error!("Failed to create run function: {}", e);
            anyhow::anyhow!("Failed to create run function")
        })?;
    globals.set("run_async", run_async_fn).map_err(|e| {
        tracing::error!("Failed to set run function in globals: {}", e);
        anyhow::anyhow!("Failed to set run function in globals")
    })?;

    Ok(())
}

fn run_lua_tasks_async(
    code: String,
    main_dir: std::path::PathBuf,
    func_names: Vec<String>,
    amount: u32,
    delay: std::time::Duration,
) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let tasks = (0..amount).flat_map(|task_id| {
            let code = code.clone();
            let main_dir = main_dir.clone();
            let func_names = func_names.clone();
            func_names.into_iter().map(move |func_name| {
                let code = code.clone();
                let main_dir = main_dir.clone();
                let func_name = func_name.clone();

                let result = tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
                    tracing::info!(
                        "Running task {}-{}: {}",
                        func_name.clone(),
                        task_id,
                        func_name
                    );

                    let mut lua_runtime = runtime::LuaRuntime::builder()
                        .with_script(code)
                        .with_main_dir(main_dir)
                        .libs()
                        .build()
                        .map_err(|e| {
                            tracing::error!("Failed to create Lua runtime: {}", e);
                            anyhow::anyhow!("Failed to create Lua runtime")
                        })?;
                    lua_runtime.run_definition(func_name.clone())?;

                    tracing::info!("Task {}-{} done", func_name.clone(), task_id);

                    Ok(())
                });

                std::thread::sleep(delay);

                result
            })
        });

        let results = futures::future::join_all(tasks).await;

        for res in results {
            res.map_err(|e| {
                tracing::error!("Task failed: {}", e);
                anyhow::anyhow!("Task failed: {}", e)
            })
            .unwrap()
            .map_err(|e| {
                tracing::error!("Task failed: {}", e);
                anyhow::anyhow!("Task failed")
            })
            .unwrap();
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua::LuaRuntime;

    #[test]
    fn test_run_single() {
        let script = r#"
            define({
                name = "run_this",
                func = function() end,
            })

            run("run_this")
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .build();
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_run_multiple() {
        let script = r#"
            define({
                name = "run_this",
                func = function() end,
            })

            run("run_this", 3)
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .build();
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_run_async_single() {
        let script = r#"
            define({
                name = "run_this",
                func = function() end,
            })

            define({
                name = "run",
                func = function()
                    run_async({ "run_this" })
                end,
            })
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .build();
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_run_async_multiple() {
        let script = r#"
            define({
                name = "run_this",
                func = function() end,
            })

            define({
                name = "run",
                func = function()
                    run_async({ "run_this" }, 3)
                end,
            })
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .build();
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_run_async_with_delay() {
        let script = r#"
            define({
                name = "run_this",
                func = function() end,
            })

            define({
                name = "run",
                func = function()
                    run_async({ "run_this" }, 3, 100)
                end,
            })
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .build();
        assert!(runtime.is_ok());
    }
}

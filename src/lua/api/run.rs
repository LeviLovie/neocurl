use super::super::runtime;

pub fn reg(
    lua: &mlua::Lua,
    registry: crate::lua::RequestRegistry,
    file_contents: String,
) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_run(lua, registry.clone())?;
    reg_run_async(lua, file_contents)?;

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

fn reg_run_async(lua: &mlua::Lua, file_contents: String) -> anyhow::Result<()> {
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

                run_lua_tasks_async(file_contents.clone(), names, amount, delay).map_err(|e| {
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
    func_names: Vec<String>,
    amount: u32,
    delay: std::time::Duration,
) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let tasks = (0..amount).flat_map(|task_id| {
            let code = code.clone();
            let func_names = func_names.clone();
            func_names.into_iter().map(move |func_name| {
                let code = code.clone();
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
                anyhow::anyhow!("Task failed")
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

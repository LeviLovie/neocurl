pub fn reg(
    lua: &mlua::Lua,
    registry: crate::registry::RequestRegistry,
    file_contents: String,
) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_run(lua, registry.clone())?;
    reg_run_async(lua, file_contents)?;

    crate::api::test::reg(lua)?;
    Ok(())
}

fn reg_run(lua: &mlua::Lua, registry: crate::registry::RequestRegistry) -> anyhow::Result<()> {
    let span = tracing::debug_span!("reg_run");
    let _enter = span.enter();

    let globals = lua.globals();
    let run_fn = lua
        .create_function(move |_, (name, amount): (String, Option<u32>)| {
            let amount = if let Some(amount) = amount { amount } else { 1 };
            tracing::info!("Running request: {} ({})", name, amount);

            for _ in 0..amount {
                crate::run_request::run(registry.clone(), name.clone()).map_err(|e| {
                    tracing::error!("Failed to run request: {}", e);
                    mlua::prelude::LuaError::runtime("Failed to run request")
                })?;
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
        .create_function(move |_, (name, amount, delay): (mlua::Table, Option<u32>, Option<u64>)| {
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
        })
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
            func_names.iter().map(move |func_id| {
                let code = code.clone();
                let func_name = func_id.clone();

                tracing::info!("Running task {}-{}: {}", func_id, task_id, func_name);

                let result = tokio::task::spawn_blocking(move || {
                    let registry: crate::registry::RequestRegistry =
                        std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

                    let lua = mlua::Lua::new();
                    if let Err(e) = crate::api::reg(&lua, registry.clone(), code.clone()) {
                        tracing::error!("Failed to initialize Lua: {}", e);
                        return Err(mlua::prelude::LuaError::runtime("Failed to initialize Lua"));
                    }

                    lua.load(&code).exec()?;

                    let registry = registry.lock().unwrap();
                    for reg in registry.iter() {
                        let name: String = reg.get("name").unwrap_or_default();
                        if name == func_name {
                            let func: mlua::Function = reg.get("func").map_err(|e| {
                                tracing::error!("Failed to get function: {}", e);
                                mlua::prelude::LuaError::runtime("Failed to get function")
                            })?;
                            func.call::<()>(()).map_err(|e| {
                                tracing::error!("Failed to call function: {}", e);
                                mlua::prelude::LuaError::runtime("Failed to call function")
                            })?;
                        }
                    }

                    tracing::info!("Task {}-{} done", func_name, task_id);
                    Ok(())
                });

                std::thread::sleep(delay);

                result
            })
        });

        // Wait for all to complete
        let results = futures::future::join_all(tasks).await;

        // Check for errors
        for res in results {
            res.map_err(|e| {
                tracing::error!("Failed to run task: {}", e);
                mlua::prelude::LuaError::runtime("Failed to run task")
            })
            .unwrap()
            .map_err(|e| {
                tracing::error!("Failed to run task: {}", e);
                mlua::prelude::LuaError::runtime("Failed to run task")
            })
            .unwrap();
        }
    });

    Ok(())
}

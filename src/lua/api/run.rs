use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[tracing::instrument]
pub fn reg(
    lua: &mlua::Lua,
    registry: crate::lua::RequestRegistry,
    file_contents: String,
    main_dir: std::path::PathBuf,
) -> anyhow::Result<()> {
    reg_run(lua, registry.clone())?;
    reg_run_async(lua, file_contents, main_dir.clone())?;

    Ok(())
}

#[tracing::instrument]
fn reg_run(lua: &mlua::Lua, registry: crate::lua::RequestRegistry) -> anyhow::Result<()> {
    let run_fn = lua.create_function(
        move |_, (name, amount, progress): (String, Option<u32>, Option<bool>)| {
            let amount = amount.unwrap_or(1);
            let progress = if amount > 1 {
                progress.unwrap_or(true)
            } else {
                false
            };
            tracing::info!("Running request: {} ({})", name, amount);

            let progress = if progress {
                let progress = MultiProgress::new();
                let style = ProgressStyle::with_template(
                    "{msg:>10} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>5}/{len:5}",
                )
                .unwrap()
                .progress_chars("##-");

                let progress_bar = progress.add(ProgressBar::new(amount.into()));
                progress_bar.set_style(style.clone());
                progress_bar.set_message("Finished:");

                Some(progress_bar)
            } else {
                None
            };

            for _ in 0..amount {
                crate::lua::runtime::run_definition_in_registry(registry.clone(), name.clone())
                    .map_err(|e| {
                        tracing::error!("Failed to run request: {}", e);
                        mlua::prelude::LuaError::runtime("Failed to run request")
                    })?;
                if let Some(ref progress) = progress {
                    progress.inc(1);
                }
            }

            if let Some(ref progress) = progress {
                progress.finish();
            }

            Ok(())
        },
    )?;
    lua.globals().set("run", run_fn)?;

    Ok(())
}

#[tracing::instrument]
fn reg_run_async(
    lua: &mlua::Lua,
    file_contents: String,
    main_dir: std::path::PathBuf,
) -> anyhow::Result<()> {
    let run_async_fn = lua.create_function(
        move |_,
              (name, amount, progress, delay): (
            mlua::Table,
            Option<u32>,
            Option<bool>,
            Option<u64>,
        )| {
            let amount = amount.unwrap_or(1);

            let names = name
                .pairs()
                .filter_map(|pair| {
                    let (_, value): (String, String) = pair.ok()?;
                    Some(value)
                })
                .collect::<Vec<String>>();

            let progress = if amount + names.len() as u32 > 1 {
                progress.unwrap_or(true)
            } else {
                false
            };

            let delay = if let Some(delay) = delay {
                std::time::Duration::from_millis(delay)
            } else {
                std::time::Duration::from_millis(100)
            };

            println!("Names: {:?}", names);

            run_lua_tasks_async(
                file_contents.clone(),
                main_dir.clone(),
                names,
                amount,
                delay,
                progress,
            )
            .map_err(|e| {
                tracing::error!("Failed to run request: {}", e);
                mlua::prelude::LuaError::runtime("Failed to run request")
            })?;

            Ok(())
        },
    )?;
    lua.globals().set("run_async", run_async_fn)?;

    Ok(())
}

fn run_lua_tasks_async(
    code: String,
    main_dir: std::path::PathBuf,
    func_names: Vec<String>,
    amount: u32,
    delay: std::time::Duration,
    progress: bool,
) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;

    let progress_bar = if progress {
        let progress = MultiProgress::new();
        let style = ProgressStyle::with_template(
            "{msg:>10} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>5}/{len:5}",
        )
        .unwrap()
        .progress_chars("##-");

        let progress_bar = progress.add(ProgressBar::new(amount.into()));
        progress_bar.set_style(style.clone());
        progress_bar.set_message("Finished:");

        Some(progress_bar)
    } else {
        None
    };

    rt.block_on(async {
        let tasks = (0..amount).flat_map(|task_id| {
            let code = code.clone();
            let main_dir = main_dir.clone();
            let func_names = func_names.clone();
            let progress_bar = progress_bar.clone();

            func_names.into_iter().map(move |func_name| {
                let code = code.clone();
                let main_dir = main_dir.clone();
                let func_name = func_name.clone();
                let progress_bar = progress_bar.clone();

                let result = tokio::task::spawn_blocking(move || -> anyhow::Result<()> {
                    tracing::info!(
                        "Running task {}-{}: {}",
                        func_name.clone(),
                        task_id,
                        func_name
                    );

                    let mut lua_runtime = crate::lua::runtime::LuaRuntime::builder()
                        .with_script(code)
                        .with_main_dir(main_dir)
                        .with_thread(format!("{}-{}", func_name, task_id))
                        .libs()
                        .build()
                        .map_err(|e| {
                            tracing::error!("Failed to create Lua runtime: {}", e);
                            anyhow::anyhow!("Failed to create Lua runtime")
                        })?;
                    lua_runtime.run_definition(func_name.clone())?;

                    if let Some(ref pb) = progress_bar {
                        pb.inc(1);
                    }
                    tracing::info!("Task {}-{} done", func_name.clone(), task_id);

                    Ok(())
                });

                std::thread::sleep(delay);

                result
            })
        });

        let results = futures::future::join_all(tasks).await;

        if let Some(ref pb) = progress_bar {
            pb.finish();
        }

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
            .with_main_dir(".".into())
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
            .with_main_dir(".".into())
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
            .with_main_dir(".".into())
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
            .with_main_dir(".".into())
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
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }
}

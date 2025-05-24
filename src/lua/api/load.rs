pub fn reg(lua: &mlua::Lua, main_dir: std::path::PathBuf) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_import(lua, main_dir.clone())?;
    reg_load(lua, main_dir.clone())?;
    reg_download(lua)?;

    Ok(())
}

fn reg_load(lua: &mlua::Lua, main_dir: std::path::PathBuf) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_load");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_load = lua
        .create_function(move |_, path: String| {
            let path = main_dir.join(path);
            match path.exists() {
                true => {
                    let content = std::fs::read_to_string(&path)?;
                    Ok(content)
                }
                false => {
                    tracing::error!("File not found: {}", path.display());
                    Err(mlua::Error::RuntimeError("File not found".to_string()))
                }
            }
        })
        .map_err(|e| {
            tracing::error!("Failed to create request function: {}", e);
            anyhow::anyhow!("Failed to create request function")
        })?;
    globals.set("load", fn_load).map_err(|e| {
        tracing::error!("Failed to set request function in globals: {}", e);
        anyhow::anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

fn reg_download(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_download");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_download = lua
        .create_function(move |_, url: String| {
            let response = match reqwest::blocking::get(&url) {
                Ok(response) => response,
                Err(e) => {
                    tracing::error!("Failed to download file: {}", e);
                    return Err(mlua::Error::RuntimeError(
                        "Failed to download file".to_string(),
                    ));
                }
            };

            match response.status() {
                reqwest::StatusCode::OK => {
                    let content = match response.bytes() {
                        Ok(content) => content,
                        Err(e) => {
                            tracing::error!("Failed to read response: {}", e);
                            return Err(mlua::Error::RuntimeError(
                                "Failed to read response".to_string(),
                            ));
                        }
                    };
                    let content_str = String::from_utf8_lossy(&content);
                    Ok(content_str.to_string())
                }
                _ => {
                    tracing::error!("Failed to download file: {}", url);
                    Err(mlua::Error::RuntimeError(
                        "Failed to download file".to_string(),
                    ))
                }
            }
        })
        .map_err(|e| {
            tracing::error!("Failed to create request function: {}", e);
            anyhow::anyhow!("Failed to create request function")
        })?;
    globals.set("download", fn_download).map_err(|e| {
        tracing::error!("Failed to set request function in globals: {}", e);
        anyhow::anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

fn reg_import(lua: &mlua::Lua, main_dir: std::path::PathBuf) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_import");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_import = lua
        .create_function(move |lua, path: String| {
            println!("Importing: {}", path);
            let abs_path = main_dir.join(&path);
            tracing::info!("Importing file: {}", abs_path.display());
            let code = match std::fs::read_to_string(&abs_path) {
                Ok(code) => code,
                Err(e) => {
                    tracing::error!("Failed to read file: {}", e);
                    return Err(mlua::Error::RuntimeError("File not found".to_string()));
                }
            };

            let chunk = lua.load(&code).set_name(&path);
            chunk.eval::<mlua::Value>()?;

            Ok(())
        })
        .map_err(|e| {
            tracing::error!("Failed to create request function: {}", e);
            anyhow::anyhow!("Failed to create request function")
        })?;
    globals.set("import", fn_import).map_err(|e| {
        tracing::error!("Failed to set request function in globals: {}", e);
        anyhow::anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

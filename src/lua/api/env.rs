pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_env(lua)?;

    Ok(())
}

fn reg_env(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_env");
    let _enter = span.enter();
    let globals = lua.globals();

    let fn_env = lua
        .create_function(|_, key: String| {
            dotenv::dotenv().ok();
            match dotenv::var(&key) {
                Ok(value) => Ok(value),
                Err(e) => {
                    tracing::error!("Failed to get env variable {}: {}", key, e);
                    Err(mlua::prelude::LuaError::runtime(
                        "Failed to get env variable",
                    ))
                }
            }
        })
        .map_err(|e| {
            tracing::error!("Failed to create env function: {}", e);
            anyhow::anyhow!("Failed to create env function")
        })?;
    globals.set("env", fn_env).map_err(|e| {
        tracing::error!("Failed to set env function: {}", e);
        anyhow::anyhow!("Failed to set env function")
    })?;

    Ok(())
}

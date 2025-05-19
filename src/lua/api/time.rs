pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_import(lua)?;

    Ok(())
}

fn reg_import(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_import");
    let _enter = span.enter();
    let globals = lua.globals();

    let fn_import = lua
        .create_function(move |lua, path: String| {
            let code = std::fs::read_to_string(&path)?;

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

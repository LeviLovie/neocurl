pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_time(lua)?;
    reg_format_time(lua)?;

    Ok(())
}

fn reg_time(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_time");
    let _enter = span.enter();
    let globals = lua.globals();

    let fn_time = lua
        .create_function(move |_, ()| {
            let now = chrono::Utc::now();
            let timestamp = now.timestamp_millis();
            let timestamp_str = format!("{}", timestamp);
            Ok(timestamp_str)
        })
        .map_err(|e| {
            tracing::error!("Failed to create request function: {}", e);
            anyhow::anyhow!("Failed to create request function")
        })?;
    globals.set("time", fn_time).map_err(|e| {
        tracing::error!("Failed to set request function in globals: {}", e);
        anyhow::anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

fn reg_format_time(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_format_time");
    let _enter = span.enter();
    let globals = lua.globals();

    let fn_format_time = lua
        .create_function(move |_, format_str: String| {
            let timestamp = chrono::Utc::now().timestamp_millis();
            let dt = match chrono::DateTime::from_timestamp_millis(timestamp) {
                None => {
                    tracing::error!("Failed to convert timestamp to NaiveDateTime");
                    return Err(mlua::prelude::LuaError::RuntimeError(format!(
                        "Failed to convert timestamp to NaiveDateTime",
                    )));
                }
                Some(dt) => dt,
            };
            tracing::info!("Current timestamp: {}", timestamp);

            let formatted_time = dt.format(&format_str).to_string();
            tracing::info!("Formatted time: {}", formatted_time);
            Ok(formatted_time)
        })
        .map_err(|e| {
            tracing::error!("Failed to create request function: {}", e);
            anyhow::anyhow!("Failed to create request function")
        })?;
    globals.set("format_time", fn_format_time).map_err(|e| {
        tracing::error!("Failed to set request function in globals: {}", e);
        anyhow::anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

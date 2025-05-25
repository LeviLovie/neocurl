#[tracing::instrument]
pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    reg_time(lua)?;
    reg_format_time(lua)?;

    Ok(())
}

#[tracing::instrument]
fn reg_time(lua: &mlua::Lua) -> anyhow::Result<()> {
    let fn_time = lua.create_function(move |_, ()| {
        let now = chrono::Utc::now();
        let timestamp = now.timestamp_millis();
        let timestamp_str = format!("{}", timestamp);

        Ok(timestamp_str)
    })?;
    lua.globals().set("time", fn_time)?;

    Ok(())
}

#[tracing::instrument]
fn reg_format_time(lua: &mlua::Lua) -> anyhow::Result<()> {
    let fn_format_time = lua.create_function(move |_, format_str: String| {
        let timestamp = chrono::Utc::now().timestamp_millis();
        let dt = match chrono::DateTime::from_timestamp_millis(timestamp) {
            None => {
                tracing::error!("Failed to convert timestamp to NaiveDateTime");
                return Err(mlua::prelude::LuaError::RuntimeError(
                    "Failed to convert timestamp to NaiveDateTime".to_string(),
                ));
            }
            Some(dt) => dt,
        };

        let formatted_time = dt.format(&format_str).to_string();
        Ok(formatted_time)
    })?;
    lua.globals().set("format_time", fn_format_time)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua::LuaRuntime;

    #[test]
    fn test_time() {
        let script = r#"
            time()
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }

    #[test]
    fn test_format_time() {
        let script = r#"
            format_time("%Y-%m-%d %H:%M:%S")
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }
}

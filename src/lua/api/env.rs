#[tracing::instrument]
pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    reg_env(lua)?;

    Ok(())
}

#[tracing::instrument]
fn reg_env(lua: &mlua::Lua) -> anyhow::Result<()> {
    let fn_env = lua.create_function(|_, key: String| {
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
    })?;
    lua.globals().set("env", fn_env)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua::LuaRuntime;

    #[test]
    fn test_env() {
        let script = r#"
            env("HOME")
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }
}

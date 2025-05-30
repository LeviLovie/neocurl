#[tracing::instrument]
pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    reg_env(lua)?;

    Ok(())
}

#[tracing::instrument]
fn reg_env(lua: &mlua::Lua) -> anyhow::Result<()> {
    let fn_env = lua.create_function(|lua, key: String| {
        dotenv::dotenv().ok();
        match dotenv::var(&key) {
            Ok(value) => {
                let string = lua.create_string(&value)?;
                Ok(mlua::Value::String(string))
            }
            Err(_) => Ok(mlua::Value::Nil),
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

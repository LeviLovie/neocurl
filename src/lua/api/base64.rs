use base64::prelude::*;

pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_to_base64(lua)?;
    reg_from_base64(lua)?;

    Ok(())
}

fn reg_to_base64(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_to_base64");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_base64 = lua
        .create_function(move |_, data: String| {
            let encoded = BASE64_STANDARD.encode(data);

            Ok(encoded)
        })
        .map_err(|e| {
            tracing::error!("Failed to create request function: {}", e);
            anyhow::anyhow!("Failed to create request function")
        })?;
    globals.set("to_base64", fn_base64).map_err(|e| {
        tracing::error!("Failed to set request function in globals: {}", e);
        anyhow::anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

fn reg_from_base64(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_from_base64");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_base64 = lua
        .create_function(move |_, data: String| {
            let decoded = BASE64_STANDARD.decode(data).unwrap_or_else(|_| {
                tracing::error!("Failed to decode base64");
                String::new().into()
            });
            let decoded = String::from_utf8(decoded).unwrap_or_else(|_| {
                tracing::error!("Failed to convert decoded data to string");
                String::new()
            });

            Ok(decoded)
        })
        .map_err(|e| {
            tracing::error!("Failed to create request function: {}", e);
            anyhow::anyhow!("Failed to create request function")
        })?;
    globals.set("from_base64", fn_base64).map_err(|e| {
        tracing::error!("Failed to set request function in globals: {}", e);
        anyhow::anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua::LuaRuntime;

    #[test]
    fn base64() {
        let script = r#"
            local data = "Hello, World!"
            local encoded = to_base64(data)
            local decoded = from_base64(encoded)
            assert(data == decoded, function()
                error("Base64 encoding/decoding failed")
            end)
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();
        assert!(runtime.is_ok());

        let runtime = runtime.unwrap();
        let (passed, failed) = runtime.test_summary();
        assert_eq!(passed, 1);
        assert_eq!(failed, 0);
    }
}

pub const LIB_DKJSON: &str = include_str!("../libs/dkjson.lua");

pub fn load(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("load");
    let _enter = span.enter();

    {
        let register = format!(
            r#"
        package = package or {{}}
        package.preload = package.preload or {{}}
        package.preload["json"] = function(...)
            {}
        end
        "#,
            LIB_DKJSON
        );
        lua.load(register)
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to load dkjson: {}", e))?;
    }

    Ok(())
}

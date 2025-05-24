const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tracing::instrument]
pub fn reg(lua: &mlua::Lua) -> mlua::Result<()> {
    reg_check_version(lua)?;

    Ok(())
}

#[tracing::instrument]
pub fn reg_check_version(lua: &mlua::Lua) -> mlua::Result<()> {
    let check_version_fn = lua.create_function(|_, version: String| {
        let script_parts: Vec<&str> = version.split('.').collect();
        if script_parts.len() < 2 || script_parts.len() > 3 {
            tracing::error!(
                "Invalid script version format: {}. Expected format is 'major.minor' or 'major.minor.patch'.",
                version
            );
            std::process::exit(1);
        }

        let script_major = script_parts[0].parse::<u32>().unwrap_or(0);
        let script_minor = script_parts[1].parse::<u32>().unwrap_or(0);
        let script_patch: Option<u32> = if script_parts.len() == 3 && script_parts[2] != "*" {
            Some(script_parts[2].parse::<u32>().unwrap_or(0))
        } else {
            None
        };

        let current_parts: Vec<&str> = VERSION.split('.').collect();
        let current_major = current_parts[0].parse::<u32>().unwrap_or(0);
        let current_minor = current_parts[1].parse::<u32>().unwrap_or(0);
        let current_patch = current_parts[2].parse::<u32>().unwrap_or(0);

        tracing::debug!(
            "Script version: {}.{}.{}, Current version: {}.{}.{}",
            script_major,
            script_minor,
            script_patch.map_or("*".to_string(), |p| p.to_string()),
            current_major,
            current_minor,
            current_patch
        );

        if script_major != current_major || script_minor != current_minor || (script_patch.is_some() && script_patch != Some(current_patch)) {
            tracing::error!(
                "Incompatible script version: expected {}.{}.{}, got {}. Try installing with version specified: `cargo install neocurl@{}`, or updating the script.",
                current_major,
                current_minor,
                current_patch,
                version,
                version,
            );
            std::process::exit(1);
        }


        Ok(())
    })?;
    lua.globals().set("check_version", check_version_fn)?;

    Ok(())
}

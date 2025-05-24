const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tracing::instrument]
pub fn reg(lua: &mlua::Lua) -> mlua::Result<()> {
    reg_check_version(lua)?;

    Ok(())
}

fn check_version_matches(current_version: String, version: String) -> bool {
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

    let current_parts: Vec<&str> = current_version.split('.').collect();
    let current_major = current_parts[0].parse::<u32>().unwrap_or(0);
    let current_minor = current_parts[1].parse::<u32>().unwrap_or(0);
    let current_patch = current_parts[2].parse::<u32>().unwrap_or(0);

    if script_major != current_major
        || script_minor != current_minor
        || (script_patch.is_some() && script_patch != Some(current_patch))
    {
        return false;
    }

    true
}

#[tracing::instrument]
pub fn reg_check_version(lua: &mlua::Lua) -> mlua::Result<()> {
    let check_version_fn = lua.create_function(|_, version: String| {
        if !check_version_matches(VERSION.to_string(), version.clone()) {
                        tracing::error!(
                "Incompatible script version: expected {}, got {}. Try installing with version specified: `cargo install neocurl@{}`, or updating the script.",
                VERSION,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_version_matches() {
        assert!(check_version_matches(
            "1.0.0".to_string(),
            "1.0.0".to_string()
        ));
        assert!(check_version_matches(
            "1.0.1".to_string(),
            "1.0.*".to_string()
        ));
        assert!(check_version_matches(
            "1.0.0".to_string(),
            "1.0.*".to_string()
        ));

        assert!(!check_version_matches(
            "1.0.0".to_string(),
            "1.1.0".to_string()
        ));
        assert!(!check_version_matches(
            "1.0.0".to_string(),
            "2.0.0".to_string()
        ));
        assert!(!check_version_matches(
            "1.1.0".to_string(),
            "1.0.*".to_string()
        ));
    }
}

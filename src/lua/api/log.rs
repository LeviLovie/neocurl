use owo_colors::{OwoColorize, XtermColors};

#[tracing::instrument]
pub fn reg(lua: &mlua::Lua, thread_name: String) -> anyhow::Result<()> {
    reg_debug(lua, thread_name.clone())?;
    reg_info(lua, thread_name.clone())?;
    reg_warn(lua, thread_name.clone())?;
    reg_error(lua, thread_name.clone())?;

    Ok(())
}

#[tracing::instrument]
fn reg_debug(lua: &mlua::Lua, thread_name: String) -> anyhow::Result<()> {
    let fn_debug = lua.create_function(move |_, msg: String| {
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
        let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();

        println!(
            "{} {} {}{} {}",
            formatted_time.color(XtermColors::DarkGray),
            "DEBUG".bright_cyan().bold(),
            thread_name.clone().color(XtermColors::DarkGray),
            ":".color(XtermColors::DarkGray),
            msg.trim()
        );

        Ok(())
    })?;
    lua.globals().set("debug", fn_debug)?;

    Ok(())
}

#[tracing::instrument]
fn reg_info(lua: &mlua::Lua, thread_name: String) -> anyhow::Result<()> {
    let fn_info = lua.create_function(move |_, msg: String| {
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
        let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();

        println!(
            "{} {} {}{} {}",
            formatted_time.color(XtermColors::DarkGray),
            " INFO".bright_green().bold(),
            thread_name.clone().color(XtermColors::DarkGray),
            ":".color(XtermColors::DarkGray),
            msg.trim()
        );

        Ok(())
    })?;
    lua.globals().set("info", fn_info)?;

    Ok(())
}

#[tracing::instrument]
fn reg_warn(lua: &mlua::Lua, thread_name: String) -> anyhow::Result<()> {
    let fn_warn = lua.create_function(move |_, msg: String| {
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
        let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();

        println!(
            "{} {} {}{} {}",
            formatted_time.color(XtermColors::DarkGray),
            " WARN".bright_yellow().bold(),
            thread_name.clone().color(XtermColors::DarkGray),
            ":".color(XtermColors::DarkGray),
            msg.trim()
        );

        Ok(())
    })?;
    lua.globals().set("warn", fn_warn)?;

    Ok(())
}

#[tracing::instrument]
fn reg_error(lua: &mlua::Lua, thread_name: String) -> anyhow::Result<()> {
    let fn_error = lua.create_function(move |_, msg: String| {
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
        let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();

        println!(
            "{} {} {}{} {}",
            formatted_time.color(XtermColors::DarkGray),
            "ERROR".bright_red().bold(),
            thread_name.clone().color(XtermColors::DarkGray),
            ":".color(XtermColors::DarkGray),
            msg.trim()
        );

        Ok(())
    })?;
    lua.globals().set("error", fn_error)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua::LuaRuntime;

    #[test]
    fn debug() {
        let script = r#"
            debug("This is a debug message")
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }

    #[test]
    fn info() {
        let script = r#"
            info("This is an info message")
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }

    #[test]
    fn warn() {
        let script = r#"
            warn("This is a warning message")
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }

    #[test]
    fn error() {
        let script = r#"
            error("This is an error message")
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }
}

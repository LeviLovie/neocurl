use anyhow::{Result, anyhow};
use mlua::{Lua, prelude::LuaError};
use tracing::error;

pub fn register(lua: &Lua) -> Result<()> {
    let span = tracing::info_span!("lua_register_helpers");
    let _enter = span.enter();

    reg_time(lua)?;
    reg_log(lua)?;

    Ok(())
}

fn reg_time(lua: &Lua) -> Result<()> {
    let span = tracing::info_span!("lua_reg_timestamp");
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
            error!("Failed to create request function: {}", e);
            anyhow!("Failed to create request function")
        })?;
    globals.set("time", fn_time).map_err(|e| {
        error!("Failed to set request function in globals: {}", e);
        anyhow!("Failed to set request function in globals")
    })?;

    let fn_format_time = lua
        .create_function(move |_, format_str: String| {
            let timestamp = chrono::Utc::now().timestamp_millis();
            let dt = match chrono::DateTime::from_timestamp_millis(timestamp) {
                None => {
                    error!("Failed to convert timestamp to NaiveDateTime");
                    return Err(LuaError::RuntimeError(format!(
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
            error!("Failed to create request function: {}", e);
            anyhow!("Failed to create request function")
        })?;
    globals.set("format_time", fn_format_time).map_err(|e| {
        error!("Failed to set request function in globals: {}", e);
        anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

fn reg_log(lua: &Lua) -> Result<()> {
    let span = tracing::info_span!("lua_reg_log");
    let _enter = span.enter();

    let globals = lua.globals();

    let fn_debug = lua
        .create_function(move |_, msg: String| {
            let timestamp = chrono::Utc::now().timestamp_millis();
            let dt = match chrono::DateTime::from_timestamp_millis(timestamp) {
                None => {
                    error!("Failed to convert timestamp to NaiveDateTime");
                    return Err(LuaError::RuntimeError(format!(
                        "Failed to convert timestamp to NaiveDateTime",
                    )));
                }
                Some(dt) => dt,
            };
            let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
            println!("{} <LUA>    DEBUG: {}", formatted_time, msg.trim());
            Ok(())
        })
        .map_err(|e| {
            error!("Failed to create request function: {}", e);
            anyhow!("Failed to create request function")
        })?;
    globals.set("debug", fn_debug).map_err(|e| {
        error!("Failed to set request function in globals: {}", e);
        anyhow!("Failed to set request function in globals")
    })?;

    let fn_info = lua
        .create_function(move |_, msg: String| {
            let timestamp = chrono::Utc::now().timestamp_millis();
            let dt = match chrono::DateTime::from_timestamp_millis(timestamp) {
                None => {
                    error!("Failed to convert timestamp to NaiveDateTime");
                    return Err(LuaError::RuntimeError(format!(
                        "Failed to convert timestamp to NaiveDateTime",
                    )));
                }
                Some(dt) => dt,
            };
            let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
            println!("{} <LUA>    INFO: {}", formatted_time, msg.trim());
            Ok(())
        })
        .map_err(|e| {
            error!("Failed to create request function: {}", e);
            anyhow!("Failed to create request function")
        })?;
    globals.set("info", fn_info).map_err(|e| {
        error!("Failed to set request function in globals: {}", e);
        anyhow!("Failed to set request function in globals")
    })?;

    let fn_warn = lua
        .create_function(move |_, msg: String| {
            let timestamp = chrono::Utc::now().timestamp_millis();
            let dt = match chrono::DateTime::from_timestamp_millis(timestamp) {
                None => {
                    error!("Failed to convert timestamp to NaiveDateTime");
                    return Err(LuaError::RuntimeError(format!(
                        "Failed to convert timestamp to NaiveDateTime",
                    )));
                }
                Some(dt) => dt,
            };
            let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
            println!("{} <LUA>    WARN: {}", formatted_time, msg.trim());
            Ok(())
        })
        .map_err(|e| {
            error!("Failed to create request function: {}", e);
            anyhow!("Failed to create request function")
        })?;
    globals.set("warn", fn_warn).map_err(|e| {
        error!("Failed to set request function in globals: {}", e);
        anyhow!("Failed to set request function in globals")
    })?;

    let fn_error = lua
        .create_function(move |_, msg: String| {
            let timestamp = chrono::Utc::now().timestamp_millis();
            let dt = match chrono::DateTime::from_timestamp_millis(timestamp) {
                None => {
                    error!("Failed to convert timestamp to NaiveDateTime");
                    return Err(LuaError::RuntimeError(format!(
                        "Failed to convert timestamp to NaiveDateTime",
                    )));
                }
                Some(dt) => dt,
            };
            let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
            println!("{} <LUA>    ERROR: {}", formatted_time, msg.trim());
            Ok(())
        })
        .map_err(|e| {
            error!("Failed to create request function: {}", e);
            anyhow!("Failed to create request function")
        })?;
    globals.set("error", fn_error).map_err(|e| {
        error!("Failed to set request function in globals: {}", e);
        anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

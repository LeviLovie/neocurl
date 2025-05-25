use std::sync::atomic::{AtomicUsize, Ordering};

use mlua::Function;
use once_cell::sync::Lazy;
use owo_colors::{OwoColorize, XtermColors};

static PASSED: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));
static FAILED: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

pub fn test_summary() -> (usize, usize) {
    let passed = PASSED.load(Ordering::Relaxed);
    let failed = FAILED.load(Ordering::Relaxed);
    println!(
        "{} {}{}{}",
        "Tests:".color(XtermColors::DarkGray),
        passed.bright_green().bold(),
        "|".color(XtermColors::DarkGray),
        failed.bright_red().bold()
    );

    PASSED.store(0, Ordering::Relaxed);
    FAILED.store(0, Ordering::Relaxed);

    (passed, failed)
}

#[tracing::instrument]
pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    reg_assert(lua)?;

    Ok(())
}

#[tracing::instrument]
fn reg_assert(lua: &mlua::Lua) -> anyhow::Result<()> {
    let fn_assert = lua.create_function(|_, (cond, msg): (bool, Option<Function>)| {
        if cond {
            PASSED.fetch_add(1, Ordering::Relaxed);
        } else {
            FAILED.fetch_add(1, Ordering::Relaxed);

            if let Some(msg) = msg {
                let result: mlua::Result<()> = msg.call(false);
                if let Err(e) = result {
                    tracing::error!("Failed to call message function: {}", e);
                }
            }
        }

        Ok(())
    })?;
    lua.globals().set("assert", fn_assert)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua::LuaRuntime;

    #[test]
    fn assert() {
        let script = r#"
            assert(true, function() error("True is not true?!") end)
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }
}

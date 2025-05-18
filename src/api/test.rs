pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_assert(lua)?;
    reg_assert_not(lua)?;
    reg_assert_eq(lua)?;
    reg_assert_ne(lua)?;

    Ok(())
}

fn reg_assert(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_test");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_assert = lua
        .create_function(|_, (title, cond): (String, bool)| {
            if cond {
                tracing::info!("Test passed: {}", title);
            } else {
                tracing::error!("Test failed: {}", title);
            }
            Ok(())
        })
        .map_err(|e| {
            tracing::error!("Failed to create assert function: {}", e);
            anyhow::anyhow!("Failed to create assert function")
        })?;
    globals.set("assert", fn_assert).map_err(|e| {
        tracing::error!("Failed to set assert function: {}", e);
        anyhow::anyhow!("Failed to set assert function")
    })?;

    Ok(())
}

fn reg_assert_not(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("req_assert_not");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_assert_not = lua
        .create_function(|_, (title, cond): (String, bool)| {
            if !cond {
                tracing::info!("Test passed: {}", title);
            } else {
                tracing::error!("Test failed: {}", title);
            }
            Ok(())
        })
        .map_err(|e| {
            tracing::error!("Failed to create assert_not function: {}", e);
            anyhow::anyhow!("Failed to create assert_not function")
        })?;
    globals.set("assert_not", fn_assert_not).map_err(|e| {
        tracing::error!("Failed to set assert_not function: {}", e);
        anyhow::anyhow!("Failed to set assert_not function")
    })?;

    Ok(())
}

fn reg_assert_eq(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_assert_eq");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_assert_eq = lua
        .create_function(|_, (title, a, b): (String, i32, i32)| {
            if a == b {
                tracing::info!("Test passed: {}", title);
            } else {
                tracing::error!("Test failed: {}", title);
            }
            Ok(())
        })
        .map_err(|e| {
            tracing::error!("Failed to create assert_eq function: {}", e);
            anyhow::anyhow!("Failed to create assert_eq function")
        })?;
    globals.set("assert_eq", fn_assert_eq).map_err(|e| {
        tracing::error!("Failed to set assert_eq function: {}", e);
        anyhow::anyhow!("Failed to set assert_eq function")
    })?;

    Ok(())
}

fn reg_assert_ne(lua: &mlua::Lua) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg_assert_ne");
    let _enter = span.enter();

    let globals = lua.globals();
    let fn_assert_ne = lua
        .create_function(|_, (title, a, b): (String, i32, i32)| {
            if a != b {
                tracing::info!("Test passed: {}", title);
            } else {
                tracing::error!("Test failed: {}", title);
            }
            Ok(())
        })
        .map_err(|e| {
            tracing::error!("Failed to create assert_ne function: {}", e);
            anyhow::anyhow!("Failed to create assert_ne function")
        })?;
    globals.set("assert_ne", fn_assert_ne).map_err(|e| {
        tracing::error!("Failed to set assert_ne function: {}", e);
        anyhow::anyhow!("Failed to set assert_ne function")
    })?;

    Ok(())
}

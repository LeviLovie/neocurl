pub fn reg(lua: &mlua::Lua, registry: crate::registry::RequestRegistry) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    reg_run(lua, registry.clone())?;

    crate::api::test::reg(lua)?;
    Ok(())
}

fn reg_run(lua: &mlua::Lua, registry: crate::registry::RequestRegistry) -> anyhow::Result<()> {
    let span = tracing::debug_span!("reg_run");
    let _enter = span.enter();

    let globals = lua.globals();
    let run_fn = lua
        .create_function(move |_, (name, amount): (String, Option<u32>)| {
            let amount = if let Some(amount) = amount { amount } else { 1 };
            tracing::info!("Running request: {} ({})", name, amount);

            for _ in 0..amount {
                crate::run_request::run(registry.clone(), name.clone()).map_err(|e| {
                    tracing::error!("Failed to run request: {}", e);
                    mlua::prelude::LuaError::runtime("Failed to run request")
                })?;
            }

            Ok(())
        })
        .map_err(|e| {
            tracing::error!("Failed to create run function: {}", e);
            anyhow::anyhow!("Failed to create run function")
        })?;
    globals.set("run", run_fn).map_err(|e| {
        tracing::error!("Failed to set run function in globals: {}", e);
        anyhow::anyhow!("Failed to set run function in globals")
    })?;

    Ok(())
}

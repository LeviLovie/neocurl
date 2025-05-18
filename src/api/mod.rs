mod log;
mod request;
mod run;
mod test;
mod time;

pub fn reg(lua: &mlua::Lua, registry: crate::registry::RequestRegistry) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    log::reg(lua)?;
    request::reg(lua, registry.clone())?;
    test::reg(lua)?;
    time::reg(lua)?;
    run::reg(lua, registry.clone())?;

    Ok(())
}

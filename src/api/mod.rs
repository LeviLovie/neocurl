mod log;
mod request;
mod test;
mod time;

pub fn reg(lua: &mlua::Lua, regystry: crate::registry::RequestRegistry) -> anyhow::Result<()> {
    let span = tracing::info_span!("lua_register_helpers");
    let _enter = span.enter();

    log::reg(lua)?;
    request::reg(lua, regystry)?;
    test::reg(lua)?;
    time::reg(lua)?;

    Ok(())
}

mod log;
mod request;
mod run;
mod test;
mod time;

pub fn reg(
    lua: &mlua::Lua,
    registry: crate::registry::RequestRegistry,
    file_contents: String,
) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    log::reg(lua)?;
    request::reg(lua, registry.clone())?;
    test::reg(lua)?;
    time::reg(lua)?;
    run::reg(lua, registry.clone(), file_contents)?;

    Ok(())
}

pub use test::test_summary;

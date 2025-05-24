mod base64;
mod dump;
mod env;
mod load;
mod log;
mod request;
mod run;
mod test;
mod time;

pub use test::test_summary;

/// Register all Lua API functions
pub fn reg(
    lua: &mlua::Lua,
    registry: crate::lua::RequestRegistry,
    file_contents: String,
    main_dir: std::path::PathBuf,
) -> anyhow::Result<()> {
    let span = tracing::info_span!("reg");
    let _enter = span.enter();

    log::reg(lua)?;
    request::reg(lua, registry.clone())?;
    test::reg(lua)?;
    time::reg(lua)?;
    run::reg(lua, registry.clone(), file_contents, main_dir.clone())?;
    base64::reg(lua)?;
    dump::reg(lua)?;
    env::reg(lua)?;
    load::reg(lua, main_dir.clone())?;

    Ok(())
}

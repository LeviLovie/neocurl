/// API for lua scripts
pub mod api;
/// Embedded Lua Libraries
pub mod libs;
/// Lua Runtime
pub mod runtime;

/// Registry to store Lua definitions
pub type RequestRegistry = std::sync::Arc<std::sync::Mutex<Vec<mlua::Table>>>;

pub use runtime::LuaRuntime;

use anyhow::{Result, anyhow};
use mlua::Function;
use tracing::error;

pub fn run(registry: crate::registry::RequestRegistry, req_name: String) -> Result<()> {
    let span = tracing::debug_span!("run");
    let _enter = span.enter();

    let registry = registry.lock().unwrap().clone();
    for req in registry.iter() {
        let name: String = req.get("name").unwrap_or_default();
        if name == req_name {
            let func: Function = req.get("func").map_err(|e| {
                error!("Failed to get function from request: {}", e);
                anyhow!("Failed to get function from request")
            })?;
            let _: () = func.call(()).map_err(|e| {
                error!("Failed to call function: {}", e);
                anyhow!("Failed to call function")
            })?;
            return Ok(());
        }
    }

    error!("No request found in registry. Run list command to see available requests.");
    return Err(anyhow!("No request found in registry"));
}

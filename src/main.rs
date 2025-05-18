mod api;
pub mod registry;
mod repl;
pub mod run_request;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use mlua::Lua;
use std::sync::{Arc, Mutex};
use tracing::{Level, debug, error, span, warn};

#[derive(Clone, Parser)]
struct Args {
    #[clap(long, short)]
    file: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Repl,
    Run { name: String },
    List,
}

fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = run() {
        error!("Error occured: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let span = tracing::info_span!("run");
    let _enter = span.enter();

    let args = Args::parse();

    let (_, file_contents) = read_file(args.clone())?;
    debug!("File read successfully");

    let registry: registry::RequestRegistry = Arc::new(Mutex::new(Vec::new()));

    let lua = Lua::new();
    api::reg(&lua, registry.clone(), file_contents.clone())?;
    debug!("Lua initialized successfully");

    lua.load(file_contents.as_str()).exec().map_err(|e| {
        anyhow!(
            "Failed to execute Lua script: {}. Error: {}",
            args.file.as_deref().unwrap_or(""),
            e
        )
    })?;
    debug!("Lua script loaded successfully");

    match args.command {
        Commands::List => {
            debug!("Listing requests");
            let registry = registry.lock().unwrap();
            for (i, req) in registry.iter().enumerate() {
                let name: String = req.get("name").unwrap_or_default();
                println!("{}: {}", i + 1, name);
            }
        }
        Commands::Run { name } => {
            debug!("Running request: {}", name);
            run_request::run(registry.clone(), name.clone())?;
        }
        Commands::Repl => {
            debug!("Starting REPL");
            repl::repl(registry.clone())?;
        }
    }

    Ok(())
}

fn read_file(args: Args) -> Result<(String, String)> {
    let span = span!(Level::INFO, "read_file", file = args.file);
    let _enter = span.enter();

    let file = args
        .file
        .clone()
        .unwrap_or_else(|| "neocurl.lua".to_string());

    let file_path = std::path::Path::new(&file);
    if !file_path.exists() {
        if args.file.is_none() {
            warn!("No file specified, using default: neocurl.lua");
        }
        error!("File not found: {}", file);
        return Err(anyhow!("No file specified"));
    }

    let file_contents = std::fs::read_to_string(file_path).map_err(|e| {
        error!("Failed to read file: {}", e);
        anyhow!("Failed to read file")
    })?;

    return Ok((file, file_contents));
}

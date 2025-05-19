mod lua;
mod repl;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use tracing::{Level, error, span, warn};

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

pub fn run() -> Result<()> {
    let span = tracing::info_span!("run");
    let _enter = span.enter();

    let args = Args::parse();

    let (_, file_contents) = read_file(args.clone())?;

    let mut runtime = lua::LuaRuntime::builder()
        .with_script(file_contents)
        .libs()
        .build()?;

    match args.command {
        Commands::List => {
            for (i, name) in runtime.list_refinitions().iter().enumerate() {
                println!("{}: {}", i + 1, name);
            }
        }
        Commands::Run { name } => {
            runtime.run_definition(name)?;

            let (_, failed) = runtime.test_summary();
            if failed > 0 {
                std::process::exit(1);
            }
        }
        Commands::Repl => {
            repl::repl(&mut runtime)?;
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

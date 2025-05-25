/// Lua Runtime
pub mod lua;
/// REPL
pub mod repl;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use tracing::{error, span, warn, Level};

/// CLI Arguments using Clap
#[derive(Clone, Parser)]
struct Args {
    #[clap(long, short)]
    file: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

/// Commands for the CLI
#[derive(Subcommand, Clone)]
enum Commands {
    Repl,
    Run { name: String },
    List,
    Test,
}

/// Main function to run the CLI
pub fn run() -> Result<()> {
    let span = tracing::info_span!("run");
    let _enter = span.enter();

    let args = Args::parse();

    let file = args
        .file
        .clone()
        .unwrap_or_else(|| "neocurl.lua".to_string());
    let main_dir = std::path::PathBuf::from(file.clone())
        .parent()
        .unwrap()
        .to_path_buf();
    let (_, file_contents) = read_file(args.clone())?;

    let mut runtime = lua::LuaRuntime::builder()
        .with_script(file_contents)
        .with_main_dir(main_dir)
        .libs()
        .build()?;

    match args.command {
        Commands::List => {
            for (i, (_, name)) in runtime.list_refinitions().iter().enumerate() {
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
        Commands::Test => {
            runtime.run_tests()?;
        }
    }

    Ok(())
}

/// Reads the file specified in the arguments
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

    Ok((file, file_contents))
}

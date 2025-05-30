pub mod api;
pub mod vm;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

/// CLI Arguments using Clap
#[derive(Clone, Parser)]
#[clap(version)]
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
        .unwrap_or_else(|| "neocurl.py".to_string());

    let vm = vm::Vm::builder()
        .load(file)
        .context("Failed to load source to VM")?
        .build()
        .context("Failed to build VM")?;

    vm.run().context("Failed to run VM")?;

    match args.command {
        Commands::List => {
            println!("Available definitions:");
            for (i, def) in api::REGISTRY.lock().unwrap().iter().enumerate() {
                println!("{}: {}", i, def.name);
            }
        }
        Commands::Run { name } => {
            vm.run_definition(name)?;
        }
        Commands::Repl => {
            // repl::repl(&mut runtime)?;
        }
        Commands::Test => {
            // runtime.run_tests()?;
        }
    }

    Ok(())
}

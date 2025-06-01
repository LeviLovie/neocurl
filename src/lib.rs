pub mod api;
pub mod vm;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use owo_colors::{OwoColorize, XtermColors};

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
            for (i, def) in vm.list_definitions().iter().enumerate() {
                println!("{}: {}", i, def);
            }
        }
        Commands::Run { name } => {
            vm.run_definition(name)?;

            let (tests_passed, tests_failed) = *api::TESTS.lock().unwrap();
            println!(
                "{} {}{}{}",
                "Test results:".color(XtermColors::DarkGray),
                tests_passed.green(),
                "/".color(XtermColors::DarkGray),
                tests_failed.red()
            );
            let (calls_passed, calls_failed) = *api::CALLS.lock().unwrap();
            println!(
                "{} {}{}{}",
                "Call results:".color(XtermColors::DarkGray),
                calls_passed.green(),
                "/".color(XtermColors::DarkGray),
                calls_failed.red()
            );

            if tests_failed > 0 || calls_failed > 0 {
                std::process::exit(1);
            }
        }
        Commands::Repl => {
            unimplemented!("REPL mode is not implemented yet");
            // repl::repl(&mut runtime)?;
        }
        Commands::Test => {
            unimplemented!("Test mode is not implemented yet");
            // runtime.run_tests()?;
        }
    }

    Ok(())
}

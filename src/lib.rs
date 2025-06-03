pub mod api;
pub mod globals;
pub mod vm;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use owo_colors::{OwoColorize, XtermColors};

const DEFAULT_FILE: &str = include_str!("default.py");

/// CLI Arguments using Clap
#[derive(Clone, Parser)]
#[clap(version)]
struct Args {
    #[clap(long, short, default_value = "ncurl.py")]
    file: String,

    #[clap(subcommand)]
    command: Commands,
}

/// Commands for the CLI
#[derive(Subcommand, Clone, Eq, PartialEq, Debug)]
enum Commands {
    Init,
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

    if args.command == Commands::Init {
        if std::path::Path::new(&args.file).exists() {
            tracing::warn!(
                "File {} already exists, skipping initialization",
                &args.file
            );
            return Ok(());
        }

        let default_file = DEFAULT_FILE
            .to_string()
            .replace("${VERSION}", env!("CARGO_PKG_VERSION"));

        std::fs::write(&args.file, default_file)
            .context(format!("Failed to write default file to {}", &args.file))?;
        println!("Initialized successfully at {}.", &args.file);

        return Ok(());
    }

    let vm = vm::Vm::builder()
        .load(args.file)
        .context("Failed to load source to VM")?
        .build()
        .context("Failed to build VM")?;

    vm.init().context("Failed to run VM")?;

    match args.command {
        Commands::List => {
            println!("Available definitions:");
            for (i, def) in vm.list_definitions().iter().enumerate() {
                println!("{}: {}", i, def);
            }
        }
        Commands::Run { name } => {
            vm.run_definition(name)?;

            let (tests_passed, tests_failed) = *globals::TESTS.lock().unwrap();
            println!(
                "{} {}{}{}",
                "Test results:".color(XtermColors::DarkGray),
                tests_passed.green(),
                "/".color(XtermColors::DarkGray),
                tests_failed.red()
            );
            let (calls_passed, calls_failed) = *globals::CALLS.lock().unwrap();
            println!(
                "{} {}{}{}",
                "Call results:".color(XtermColors::DarkGray),
                calls_passed.green(),
                "/".color(XtermColors::DarkGray),
                calls_failed.red()
            );

            vm.cleanup().context("Failed to cleanup VM")?;

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
        _ => {
            tracing::error!("Unknown command: {:?}", args.command);
            return Err(anyhow::anyhow!("Unknown command"));
        }
    }

    Ok(())
}

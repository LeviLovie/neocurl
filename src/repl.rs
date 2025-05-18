use crate::run_request;
use anyhow::Result;
use linefeed::{Interface, ReadResult};
use tracing::debug;

pub fn repl(registry: crate::registry::RequestRegistry) -> Result<()> {
    let reader = Interface::new("neocurl")?;
    reader.set_prompt(">> ")?;

    loop {
        match reader.read_line()? {
            ReadResult::Input(line) => {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                match parts[0] {
                    "list" => {
                        debug!("Listing requests");
                        let registry = registry.lock().unwrap();
                        for (i, req) in registry.iter().enumerate() {
                            let name: String = req.get("name").unwrap_or_default();
                            println!("{}: {}", i + 1, name);
                        }
                    }
                    "run" if parts.len() >= 2 => {
                        let name = parts[1].to_string();
                        debug!("Running request from REPL: {}", name);
                        if let Err(err) = run_request::run(registry.clone(), name) {
                            eprintln!("Error: {}", err);
                        }
                    }
                    "exit" | "quit" => {
                        println!("Exiting REPL.");
                        break;
                    }
                    cmd => {
                        eprintln!("Unknown command: {}", cmd);
                    }
                }
            }
            ReadResult::Eof => {
                println!("Exiting REPL.");
                break;
            }
            ReadResult::Signal(_) => {
                println!("Received signal, exiting REPL.");
                break;
            }
        }
    }

    Ok(())
}

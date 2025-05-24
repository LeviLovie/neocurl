//! Module for the REPL (Read-Eval-Print Loop) interface.

use crate::lua::LuaRuntime;
use anyhow::Result;
use linefeed::{Interface, ReadResult};

/// Starts a Read-Eval-Print Loop (REPL) for the Lua runtime.
pub fn repl(runtime: &mut LuaRuntime) -> Result<()> {
    let reader = Interface::new("ncurl")?;
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
                        for (i, (_, name)) in runtime.list_refinitions().iter().enumerate() {
                            println!("{}: {}", i + 1, name);
                        }
                    }
                    "run" if parts.len() >= 2 => {
                        let name = parts[1].to_string();

                        runtime.run_definition(name)?;
                        let _ = runtime.test_summary();
                    }
                    "test" => {
                        runtime.run_tests()?;
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

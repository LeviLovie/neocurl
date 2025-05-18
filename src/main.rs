mod lua_helpers;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use mlua::{prelude::LuaError, Function, Lua, Table};
use std::{cell::RefCell, rc::Rc};
use tracing::{debug, error, span, warn, Level};

type RequestRegistry = Rc<RefCell<Vec<Table>>>;

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
    Run { name: String, args: Vec<String> },
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

    let registry: RequestRegistry = Rc::new(RefCell::new(Vec::new()));

    let lua = Lua::new();
    lua_helpers::register(&lua)?;
    lua_reg_request(&lua, registry.clone())?;
    lua_reg_send(&lua)?;
    lua_reg_print_response(&lua)?;
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
            for (i, req) in registry.borrow().iter().enumerate() {
                let name: String = req.get("name").unwrap_or_default();
                println!("{}: {}", i + 1, name);
            }
        }
        Commands::Run { name, args } => {
            debug!("Running request: {}", name);
            run_request(registry.clone(), args.clone(), name.clone())?;
        }
        Commands::Repl => unimplemented!("REPL mode is not implemented yet"),
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

fn run_request(registry: RequestRegistry, _args: Vec<String>, req_name: String) -> Result<()> {
    let span = tracing::debug_span!("run_request");
    let _enter = span.enter();

    for req in registry.borrow().iter() {
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

fn lua_reg_request(lua: &Lua, registry: RequestRegistry) -> Result<()> {
    let span = tracing::debug_span!("lua_register_request");
    let _enter = span.enter();

    let globals = lua.globals();
    let request_fn = lua
        .create_function(move |_, req: Table| {
            registry.borrow_mut().push(req.clone());
            Ok(())
        })
        .map_err(|e| {
            error!("Failed to create request function: {}", e);
            anyhow!("Failed to create request function")
        })?;
    globals.set("request", request_fn).map_err(|e| {
        error!("Failed to set request function in globals: {}", e);
        anyhow!("Failed to set request function in globals")
    })?;

    Ok(())
}

fn lua_reg_send(lua: &Lua) -> Result<()> {
    let span = tracing::debug_span!("lua_define_request_type_functions");
    let _enter = span.enter();

    let globals = lua.globals();

    let send_fn = lua
        .create_function(|lua, args: Table| {
            let start = std::time::Instant::now();
            println!("Sending request...");

            let url: String = match args.get("url") {
                Err(e) => {
                    error!("Failed to get URL from args: {}", e);
                    return Err(LuaError::runtime("Failed to get URL from args"));
                }
                Ok(url) => url,
            };
            let method: String = match args.get("method") {
                Err(e) => {
                    error!("Failed to get method from args: {}", e);
                    return Err(LuaError::runtime("Failed to get method from args"));
                }
                Ok(method) => method,
            };
            let headers: Option<Table> = args.get("headers").ok();
            let query: Option<Table> = args.get("query").ok();
            let body: Option<String> = args.get("body").ok();

            let client = reqwest::blocking::Client::new();
            let method = match method.as_str() {
                "GET" => reqwest::Method::GET,
                "POST" => reqwest::Method::POST,
                "PUT" => reqwest::Method::PUT,
                "DELETE" => reqwest::Method::DELETE,
                _ => {
                    error!("Unsupported method: {}", method);
                    return Err(LuaError::runtime("Unsupported method"));
                }
            };

            let mut request_builder = client.request(method, &url);
            if let Some(headers) = headers {
                for pair in headers.pairs::<String, String>() {
                    if pair.is_err() {
                        error!("Failed to get headers from args: {}", pair.unwrap_err());
                        return Err(LuaError::runtime("Failed to get headers from args"));
                    }
                    let (key, value) = pair.unwrap();
                    request_builder = request_builder.header(&key, &value);
                }
            }
            if let Some(query) = query {
                for pair in query.pairs::<String, String>() {
                    if pair.is_err() {
                        error!("Failed to get query from args: {}", pair.unwrap_err());
                        return Err(LuaError::runtime("Failed to get query from args"));
                    }
                    let (key, value) = pair.unwrap();
                    request_builder = request_builder.query(&[(key, value)]);
                }
            }
            if let Some(body) = body {
                request_builder = request_builder.body(body);
            }

            let response = request_builder.send().map_err(|e| {
                error!("Failed to send request: {}", e);
                LuaError::runtime("Failed to send request")
            })?;

            let status = response.status();
            let status_code = status.as_u16();
            let status_text = status.canonical_reason().unwrap_or("Unknown");
            let headers = response.headers().clone();
            let body = response.text().map_err(|e| {
                error!("Failed to read response body: {}", e);
                LuaError::runtime("Failed to read response body")
            })?;

            let elapsed = start.elapsed();

            let response_table = lua.create_table()?;
            response_table.set("elapsed", elapsed.as_millis())?;
            response_table.set("status", status_code)?;
            response_table.set("status_text", status_text)?;
            let headers_table = lua.create_table()?;
            for (key, value) in headers.iter() {
                headers_table.set(key.as_str(), value.to_str().unwrap_or(""))?;
            }
            response_table.set("headers", headers_table)?;
            response_table.set("body", body)?;

            Ok(response_table)
        })
        .map_err(|e| {
            error!("Failed to create GET function: {}", e);
            anyhow!("Failed to create GET function")
        })?;
    globals.set("send", send_fn).map_err(|e| {
        error!("Failed to set GET function in globals: {}", e);
        anyhow!("Failed to set GET function in globals")
    })?;

    Ok(())
}

fn lua_reg_print_response(lua: &Lua) -> Result<()> {
    let span = tracing::debug_span!("lua_print_response");
    let _enter = span.enter();

    let globals = lua.globals();

    let print_response_fn = lua
        .create_function(|_, response: Table| {
            let status: u16 = response.get("status").unwrap_or_default();
            let status_text: String = response.get("status_text").unwrap_or_default();
            let headers: Option<Table> = response.get("headers").ok();
            let body: String = response.get("body").unwrap_or_default();
            let elapsed: u64 = response.get("elapsed").unwrap_or_default();

            println!("Status: {} {}", status, status_text);
            if let Some(headers) = headers {
                println!("Headers:");
                for pair in headers.pairs::<String, String>() {
                    if pair.is_err() {
                        error!("Failed to get headers from response: {}", pair.unwrap_err());
                        return Err(LuaError::runtime("Failed to get headers from response"));
                    }
                    let (key, value) = pair.unwrap();
                    println!("  {}: {}", key, value);
                }
            }
            println!("Body:");
            println!("{}", body);
            println!("Elapsed: {} ms", elapsed);

            Ok(())
        })
        .map_err(|e| {
            error!("Failed to create print_response function: {}", e);
            anyhow!("Failed to create print_response function")
        })?;
    globals
        .set("print_response", print_response_fn)
        .map_err(|e| {
            error!("Failed to set print_response function in globals: {}", e);
            anyhow!("Failed to set print_response function in globals")
        })?;

    Ok(())
}

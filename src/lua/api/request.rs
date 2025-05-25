#[tracing::instrument]
pub fn reg(lua: &mlua::Lua, registry: crate::lua::RequestRegistry) -> anyhow::Result<()> {
    reg_define(lua, registry)?;
    reg_send(lua)?;
    reg_print_response(lua)?;

    Ok(())
}

#[tracing::instrument]
fn reg_define(lua: &mlua::Lua, registry: crate::lua::RequestRegistry) -> anyhow::Result<()> {
    let define_fn = lua.create_function(move |_, req: mlua::Table| {
        let mut registry = registry.lock().unwrap();
        registry.push(req);

        Ok(())
    })?;
    lua.globals().set("define", define_fn)?;

    Ok(())
}

#[tracing::instrument]
fn reg_send(lua: &mlua::Lua) -> anyhow::Result<()> {
    let send_fn = lua.create_function(|lua, args: mlua::Table| {
        let start = std::time::Instant::now();
        tracing::info!("Sending request...");

        let url: String = match args.get("url") {
            Err(e) => {
                tracing::error!("Failed to get URL from args: {}", e);
                return Err(mlua::prelude::LuaError::runtime(
                    "Failed to get URL from args",
                ));
            }
            Ok(url) => url,
        };
        let method: String = match args.get("method") {
            Err(e) => {
                tracing::error!("Failed to get method from args: {}", e);
                return Err(mlua::prelude::LuaError::runtime(
                    "Failed to get method from args",
                ));
            }
            Ok(method) => method,
        };
        let headers: Option<mlua::Table> = args.get("headers").ok();
        let query: Option<mlua::Table> = args.get("query").ok();
        let body: Option<String> = args.get("body").ok();

        let client = reqwest::blocking::Client::new();
        let method = match method.as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            _ => {
                tracing::error!("Unsupported method: {}", method);
                return Err(mlua::prelude::LuaError::runtime("Unsupported method"));
            }
        };

        let mut request_builder = client.request(method, &url);
        if let Some(headers) = headers {
            for pair in headers.pairs::<String, String>() {
                if pair.is_err() {
                    tracing::error!("Failed to get headers from args: {}", pair.unwrap_err());
                    return Err(mlua::prelude::LuaError::runtime(
                        "Failed to get headers from args",
                    ));
                }
                let (key, value) = pair.unwrap();
                request_builder = request_builder.header(&key, &value);
            }
        }
        if let Some(query) = query {
            for pair in query.pairs::<String, String>() {
                if pair.is_err() {
                    tracing::error!("Failed to get query from args: {}", pair.unwrap_err());
                    return Err(mlua::prelude::LuaError::runtime(
                        "Failed to get query from args",
                    ));
                }
                let (key, value) = pair.unwrap();
                request_builder = request_builder.query(&[(key, value)]);
            }
        }
        if let Some(body) = body {
            request_builder = request_builder.body(body);
        }

        let response = request_builder.send().map_err(|e| {
            tracing::error!("Failed to send request: {}", e);
            mlua::prelude::LuaError::runtime("Failed to send request")
        })?;

        let status = response.status();
        let status_code = status.as_u16();
        let status_text = status.canonical_reason().unwrap_or("Unknown");
        let headers = response.headers().clone();
        let body = response.text().map_err(|e| {
            tracing::error!("Failed to read response body: {}", e);
            mlua::prelude::LuaError::runtime("Failed to read response body")
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
    })?;
    lua.globals().set("send", send_fn)?;

    Ok(())
}

#[tracing::instrument]
fn reg_print_response(lua: &mlua::Lua) -> anyhow::Result<()> {
    let print_response_fn = lua.create_function(|_, response: mlua::Table| {
        let status: u16 = response.get("status").unwrap_or_default();
        let status_text: String = response.get("status_text").unwrap_or_default();
        let headers: Option<mlua::Table> = response.get("headers").ok();
        let body: String = response.get("body").unwrap_or_default();
        let elapsed: u64 = response.get("elapsed").unwrap_or_default();

        println!("Elapsed: {} ms", elapsed);
        println!("Status: {} {}", status, status_text);
        if let Some(headers) = headers {
            println!("Headers:");
            for pair in headers.pairs::<String, String>() {
                if pair.is_err() {
                    tracing::error!("Failed to get headers from response: {}", pair.unwrap_err());
                    return Err(mlua::prelude::LuaError::runtime(
                        "Failed to get headers from response",
                    ));
                }
                let (key, value) = pair.unwrap();
                println!("  {}: {}", key, value);
            }
        }
        println!("Body:");
        println!("{}", body);

        Ok(())
    })?;
    lua.globals().set("print_response", print_response_fn)?;

    Ok(())
}

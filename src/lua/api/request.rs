use futures::FutureExt;
use mlua::prelude::*;

#[tracing::instrument]
pub fn reg(lua: &mlua::Lua, registry: crate::lua::RequestRegistry) -> anyhow::Result<()> {
    reg_define(lua, registry)?;
    reg_send(lua)?;
    reg_send_async(lua)?;
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

#[derive(Clone, Debug)]
struct Request {
    pub url: String,
    pub method: reqwest::Method,
    pub headers: reqwest::header::HeaderMap,
    pub query: Vec<(String, String)>,
    pub body: Option<String>,
    pub body_as_bytes: bool,
}

impl Request {
    pub fn from_table(table: mlua::Table) -> anyhow::Result<Self> {
        let url: String = table.get("url")?;
        let body: Option<String> = table.get("body").ok();
        let body_as_bytes: bool = table.get("body_as_bytes").unwrap_or(false);

        let method: reqwest::Method = match table.get::<String>("method") {
            Ok(method_str) => match method_str.as_str() {
                "GET" => reqwest::Method::GET,
                "POST" => reqwest::Method::POST,
                "PUT" => reqwest::Method::PUT,
                "DELETE" => reqwest::Method::DELETE,
                _ => {
                    tracing::error!("Unsupported HTTP method: {}", method_str);
                    return Err(anyhow::anyhow!("Unsupported HTTP method: {}", method_str));
                }
            },
            Err(_) => reqwest::Method::GET,
        };

        let headers: reqwest::header::HeaderMap = match table.get::<mlua::Table>("headers") {
            Ok(headers_table) => {
                let mut headers_map = reqwest::header::HeaderMap::new();
                for pair in headers_table.pairs::<String, String>() {
                    match pair {
                        Ok((key, value)) => {
                            if let Ok(header_name) =
                                reqwest::header::HeaderName::from_bytes(key.as_bytes())
                            {
                                let value = match value.parse::<reqwest::header::HeaderValue>() {
                                    Ok(v) => v,
                                    Err(e) => {
                                        tracing::warn!("Invalid header value for {}: {}", key, e);
                                        continue;
                                    }
                                };
                                headers_map.insert(header_name, value);
                            } else {
                                tracing::warn!("Invalid header name: {}", key);
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to parse header pair: {}", e);
                        }
                    }
                }
                headers_map
            }
            Err(_) => reqwest::header::HeaderMap::new(),
        };

        let query: Vec<(String, String)> = match table.get::<mlua::Table>("query") {
            Ok(query_table) => query_table
                .pairs::<String, String>()
                .filter_map(|pair| pair.ok())
                .collect(),
            Err(_) => Vec::new(),
        };

        Ok(Self {
            url,
            method,
            headers,
            query,
            body,
            body_as_bytes,
        })
    }
}

#[tracing::instrument]
fn reg_send(lua: &mlua::Lua) -> anyhow::Result<()> {
    let send_fn = lua.create_function(|lua, args: mlua::Table| {
        let start = std::time::Instant::now();
        tracing::info!("Sending request...");

        let req = Request::from_table(args).map_err(|e| {
            tracing::error!("Failed to parse request from args: {}", e);
            mlua::prelude::LuaError::runtime(format!("Failed to parse request from args: {}", e))
        })?;

        let client = reqwest::blocking::Client::new();

        let mut request_builder = client
            .request(req.method, &req.url)
            .headers(req.headers.clone());
        for (key, value) in req.query {
            request_builder = request_builder.query(&[(key, value)]);
        }
        if let Some(body) = req.body {
            request_builder = if req.body_as_bytes {
                request_builder.body(body.into_bytes())
            } else {
                request_builder.body(body)
            };
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

#[derive(Clone, Debug)]
struct ResponseStruct {
    index: u32,
    elapsed: u128,
    status: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
}

#[tracing::instrument]
fn reg_send_async(lua: &mlua::Lua) -> anyhow::Result<()> {
    let send_async_fn = lua.create_function(
        |lua, (args, amount): (mlua::Table, u32)| {
            let req = Request::from_table(args).map_err(|e| {
                tracing::error!("Failed to parse request from args: {}", e);
                mlua::prelude::LuaError::runtime(format!(
                    "Failed to parse request from args: {}",
                    e
                ))
            })?;

            tracing::info!("Sending async request...");

            type FuturesType = Vec<
                std::pin::Pin<
                    Box<dyn Future<Output = Result<ResponseStruct, LuaError>> + std::marker::Send>,
                >,
            >;

            let mut futures: FuturesType = Vec::<
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<ResponseStruct, LuaError>>
                            + std::marker::Send,
                    >,
                >,
            >::new();
            for i in 0..amount {
                let req = req.clone();
                let start = std::time::Instant::now();
                let mut request_builder = reqwest::Client::new()
                    .request(req.method, &req.url)
                    .headers(req.headers.clone());

                for (key, value) in req.query {
                    request_builder = request_builder.query(&[(key, value)]);
                }
                if let Some(body) = req.body {
                    if req.body_as_bytes {
                        request_builder = request_builder.body(body.into_bytes());
                    } else {
                        request_builder = request_builder.body(body);
                    }
                }

                let future = async move {
                    let response = request_builder.send().await.map_err(|e| {
                        tracing::error!("Failed to send request: {}", e);
                        mlua::prelude::LuaError::runtime("Failed to send request")
                    })?;

                    let status = response.status();
                    let status_code = status.as_u16();
                    let status_text = status.canonical_reason().unwrap_or("Unknown").to_string();
                    let headers = response.headers().clone();
                    let text = response.text().await.map_err(|e| {
                        tracing::error!("Failed to read response body: {}", e);
                        mlua::prelude::LuaError::runtime("Failed to read response body")
                    })?;
                    let elapsed = start.elapsed();

                    let headers_vec: Vec<(String, String)> = headers
                        .iter()
                        .filter_map(|(key, value)| {
                            value
                                .to_str()
                                .ok()
                                .map(|v| (key.as_str().to_string(), v.to_string()))
                        })
                        .collect();

                    let response = ResponseStruct {
                        index: i,
                        elapsed: elapsed.as_millis(),
                        status: status_code,
                        status_text,
                        headers: headers_vec,
                        body: text,
                    };

                    Ok(response)
                };

                futures.push(future.boxed());
            }

            let rt = tokio::runtime::Runtime::new().map_err(|e| {
                tracing::error!("Failed to create tokio runtime: {}", e);
                mlua::prelude::LuaError::runtime("Failed to create tokio runtime")
            })?;

            let mut awaited_futures = Vec::new();

            rt.block_on(async {
                awaited_futures = futures::future::join_all(futures).await;
            });

            let result = lua.create_table()?;
            for future in awaited_futures {
                let response = future.map_err(|e| {
                    tracing::error!("Failed to process response: {}", e);
                    mlua::prelude::LuaError::runtime("Failed to process response")
                })?;

                let response_table = lua.create_table()?;
                response_table.set("elapsed", response.elapsed)?;
                response_table.set("status", response.status)?;
                response_table.set("status_text", response.status_text)?;
                response_table.set("body", response.body)?;

                let headers_table = lua.create_table()?;
                for (key, value) in response.headers {
                    headers_table.set(key, value)?;
                }
                response_table.set("headers", headers_table)?;

                result.set(response.index + 1, response_table)?;
            }

            Ok(result)
        },
    )?;
    lua.globals().set("send_async", send_async_fn)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Lua;

    #[test]
    fn test_request_from_table() {
        let lua = Lua::new();
        let table = lua.create_table().unwrap();
        table.set("url", "http://example.com").unwrap();
        table.set("method", "GET").unwrap();
        table.set("body", None::<String>).unwrap();
        let request = Request::from_table(table).unwrap();
        assert_eq!(request.url, "http://example.com");
        assert_eq!(request.method, reqwest::Method::GET);
        assert!(request.body.is_none());
    }
}

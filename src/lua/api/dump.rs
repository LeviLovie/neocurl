#[tracing::instrument]
pub fn reg(lua: &mlua::Lua) -> anyhow::Result<()> {
    reg_dump(lua)?;

    Ok(())
}

fn value_to_string(value: mlua::Value) -> String {
    match value {
        mlua::Value::Nil => "nil".to_string(),
        mlua::Value::Boolean(b) => b.to_string(),
        mlua::Value::Integer(i) => i.to_string(),
        mlua::Value::Number(n) => n.to_string(),
        mlua::Value::String(s) => s
            .to_str()
            .map(|s| s.to_owned())
            .unwrap_or_else(|_| "invalid utf8".to_string()),
        mlua::Value::Table(t) => {
            let mut is_array = true;
            let mut items = vec![];

            for pair in t.clone().pairs::<mlua::Value, mlua::Value>() {
                let (k, v) = match pair {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                if let mlua::Value::Integer(i) = k {
                    if i > 0 {
                        items.push(value_to_string(v));
                        continue;
                    }
                }
                is_array = false;

                break;
            }

            if is_array {
                format!("[{}]", items.join(", "))
            } else {
                "<table>".to_string()
            }
        }
        mlua::Value::Function(_) => "<function>".to_string(),
        mlua::Value::Thread(_) => "<thread>".to_string(),
        mlua::Value::UserData(_) => "<userdata>".to_string(),
        mlua::Value::LightUserData(_) => "<lightuserdata>".to_string(),
        mlua::Value::Other(_) => "<other>".to_string(),
        mlua::Value::Error(err) => format!("<error: {}>", err),
    }
}

fn dump_table(table: mlua::Table) -> String {
    let mut result = String::new();

    for pair in table.pairs::<mlua::Value, mlua::Value>() {
        if let Err(e) = pair {
            tracing::error!("Failed to get pair from table: {}", e);
            continue;
        }
        let (key, value) = pair.unwrap();

        let key_str = match key {
            mlua::Value::String(s) => s
                .to_str()
                .map(|s| s.to_owned())
                .unwrap_or_else(|_| "invalid utf8".to_string()),
            mlua::Value::Integer(i) => i.to_string(),
            mlua::Value::Number(n) => n.to_string(),
            _ => "<unknown key>".to_string(),
        };
        let value_str = value_to_string(value);

        result.push_str(&format!("{}: {}\n", key_str, value_str));
    }

    result
}

#[tracing::instrument]
fn reg_dump(lua: &mlua::Lua) -> anyhow::Result<()> {
    let fn_dump = lua.create_function(|_, obj: mlua::Table| {
        let dump = dump_table(obj);

        Ok(dump)
    })?;
    lua.globals().set("dump", fn_dump)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua::LuaRuntime;

    #[test]
    fn test_dump() {
        let script = r#"
            local test_table = {
                key1 = "value1",
                key2 = 42,
                key3 = true,
                key4 = { nested_key = "nested_value" },
            }
            dump(test_table)
        "#;
        let runtime = LuaRuntime::builder()
            .with_script(script.to_string())
            .with_main_dir(".".into())
            .build();

        assert!(runtime.is_ok());
    }
}

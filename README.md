[![Crates.io](https://img.shields.io/crates/v/neocurl.svg)](https://crates.io/crates/neocurl)
[![Docs.rs](https://docs.rs/neocurl/badge.svg)](https://docs.rs/neocurl)
[![License](https://img.shields.io/crates/l/neocurl.svg)](LICENSE)
[![CI/CD](https://github.com/levilovie/neocurl/actions/workflows/ci.yml/badge.svg)](https://github.com/levilovie/neocurl/actions/workflows/ci.yml/)

# NeoCurl

A command line tool to test servers.

## Features

- [x] Sending requests
- [x] Running definintions at runtime
- [x] Tests
- [x] Logs
- [x] Json support
- [x] Running tests

## Install

Make sure you have [rust](https://www.rust-lang.org/learn/get-started) installed.

Install using `cargo install neocurl`.

Now `neocurl` and `ncurl` commands should be available.

## Usage

1. Create a new file (default name is `neocurl.lua`)
2. Add request definitions. Example:

```lua
define({
    name = "get_request",
    func = function()
        result = send({
            url = "https://httpbin.org/get",
            method = "GET",
            headers = {
                ["User-Agent"] = "Neocurl",
                ["Accept"] = "application/json"
            },
        })

        print_response(result)
    end,
})
```

3. Run via `ncurl run get_request`

## Advaced

Use checks, helpers, tests, runs, and async.

```lua
-- Check the NeoCurl version
check_version("1.2.3")

-- Read BASE_URL from envs
BASE_URL = env("BASE_URL") or "http://localhost:8080"

define({
    name = "get_request",
    func = function()
        -- Helper function to get and format currnet timestamp
        now = format_time("%Y-%m-%d %H:%M:%S")

        result = send({
            url = BASE_URL,
            method = "GET",
            headers = {
                ["User-Agent"] = "Neocurl",
                ["Accept"] = "application/json",
                ["X-Time"] = now
            },
        })

        print_response(result)

        -- Test for result
        assert(result.status == 200)
    end,
})

define({
    name = "run_get_request",
    -- Do not run as a test
    test = false,
    func = function()
        -- Run a definition and runtime twice
        run("get_request", 2)
    end,
})

define({
    name = "async",
    func = function()
        -- Run definitions and runtime using async.
        run_async({"get_request", "run_get_request"}, 10)
    end,
})
```

## Guide

### Args

- `--file` or `-f` and `<string>`: Set the file path. Default is `neocurl.lua`. Ex. `neocurl -f ncurl_custom_file.lua`
- Commands:
  - `list`: List all definitions from the file.
  - `run` and `<name>`: Run a definition from the file.
  - `repl`: Run REPL.

### Functions

#### Definitions

- `define({name, test, func})`: Creates a new definition with `name`, `func` is executed when called, and `test` states if it should be run as a test (default is `true`).

#### Runs

- `run(name, Option<amount>, Option<progress>)`: Run a definition by `name`. Optional `amount` specifies the amount of subsequent calls, default is 1. And optional `progress` specifies whether to draw a progress bar.
- `run_async(names, Option<amount>, Option<progress>, Option<delay>)`: Run a definitions specified in the names table (Ex. `{"run1", "run2"}`). Optional `amount` specifies the amount of subsequent calls, default is 1. Optional `delay` specifies delay between async calls in milliseconds, default is 100ms (if set too low, unexpected behaivor might occur as the amount of threads is not limited). And optional `progress` specifies whether to draw a progress bar.

### Requests

- `send({url, method, headers, query, body})`: Sends a request to `url`.
- `send_async({url, method, headers, query, body}, amount)`: Sends `amount` of requests in parallel.

#### Log

- `debug(message)`: Log `message` with debug level.
- `info(message)`: Log `message` with info level.
- `warn(message)`: Log `message` with warn level.
- `error(message)`: Log `message` with error level.

#### Test

If any test fails in `run` command, the tool will exit with exit code of `1` (Use in CI/CD? :D).

- `assert(condition, fail_func)`: Asserts that `condition` is `true`, if not, runs `fail_func`.

#### Helpers

- `time`: Returns timestamp in millis since the epoch.
- `format_time(format_str)`: Returns time formatted using format_str. Ex. `format_time("%Y-%m-%d %H:%M:%S")`
- `to_base64(payload)`: Encodes `payload` in base64.
- `from_base64(base64)`: Decodes from base64
- `dump(value)`: Dumps `value` to a string and returns it. Useful for debugging tables. Ex. `print(dump(tbl))`
- `env(var)`: Reads and returns env `var`. Returns `nil` if `var` does not exist.

TOKEN = env("TOKEN")

#### Checks

- `check_version(version)`: Checks if `version` and current NeoCurl version are the same. Version is in [Semantic Versioning](https://semver.org/) (`major.minor.patch`), setting `patch` to `*` will ignore patch check. Ex. `check_version("1.2.*")`.

#### Import

- `load(path)`: Returns content loaded from a file located at `path`. Ex. `payload = import("payload.json")`
- `download(url)`: Returns file downloaded from url.

### Built-in Libs

#### Json

[dkjson](https://dkolf.de/dkjson-lua/) is used to encode/decode json. Example usage:

```lua
define({
    name = "json",
    func = function()
        json = require("json")
        local tbl = {
            animals = { "dog", "cat", "aardvark" },
            instruments = { "violin", "trombone", "theremin" },
            bugs = json.null,
            trees = nil
        }
        local str = json.encode(tbl, { indent = false })

        info(str)
    end,
})
```

[![Crates.io](https://img.shields.io/crates/v/neocurl.svg)](https://crates.io/crates/neocurl)
[![Docs.rs](https://docs.rs/neocurl/badge.svg)](https://docs.rs/neocurl)
[![License](https://img.shields.io/crates/l/neocurl.svg)](LICENSE)
[![CI/CD](https://github.com/levilovie/neocurl/actions/workflows/ci.yml/badge.svg)](https://github.com/levilovie/neocurl/actions/workflows/ci.yml/)

# NeoCurl

A command line tool to test servers.

Read [the book](https://neocurl.lovie.dev/) for **quick start** and **guide**.

## Features

- [x] Sending requests
- [x] Running definintions at runtime
- [x] Tests
- [x] Logs
- [x] Json support
- [x] Running tests

## Example

```python
import neocurl as nc

@nc.define
def get(client):
    response = client.get("https://httpbin.org/get")
    nc.info(f"Response status: {response.status}, finished in {response.duration:.2f}ms")
    assert response.status_code == 200, f"Expected status code 200, but got {response.status_code} ({response.status})"
    response.print()
```
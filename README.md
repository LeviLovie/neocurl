[![Crates.io](https://img.shields.io/crates/v/neocurl.svg)](https://crates.io/crates/neocurl)
[![Docs.rs](https://docs.rs/neocurl/badge.svg)](https://docs.rs/neocurl)
[![License](https://img.shields.io/crates/l/neocurl.svg)](LICENSE)
[![CI/CD](https://github.com/levilovie/neocurl/actions/workflows/ci.yml/badge.svg)](https://github.com/levilovie/neocurl/actions/workflows/ci.yml/)

> [!CAUTION]
> This GitHub repository is archived and no longer updated.
>
> NeoCurl has moved to my self-hosted Forgejo instance: https://git.lovie.dev/levi/neocurl.
> A public mirror is also available on GitLab for people without local Forgejo accounts: https://gitlab.com/levilovie/neocurl.
>
> Issues and merge requests are welcome on Forgejo or GitLab. The code in this GitHub repository is out of date and should not be used as the source of truth.
>
> See https://lovie.dev/code for more info.

# NeoCurl

A command line tool to test servers.

Read [the book](https://neocurl.lovie.dev/) for **quick start** and **guide**.

## Features

- [x] Sending requests
- [x] Asserts
- [x] Logs
- [x] Third-party libs
- [x] Async requests

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

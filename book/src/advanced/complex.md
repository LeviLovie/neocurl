# Complex requests

NeoCurl supports sending complex requests via [python's keyword arguments](https://docs.python.org/3/tutorial/controlflow.html#keyword-arguments).

## Send

### Breakdown

Here is a breakdown of keyword args supported by `nc.Client.send`:

- `method: nc.Method`

  HTTP request [method](https://en.wikipedia.org/wiki/HTTP#Request_methods).
  Type is an enum defined in `neocurl` module.

  - `nc.Method.Get` or `nc.GET`
  - `nc.Method.Head` or `nc.HEAD`
  - `nc.Method.Post` or `nc.POST`
  - `nc.Method.Put` or `nc.POST`
  - `nc.Method.Delete` or `nc.DELETE`
  - `nc.Method.Patch` or `nc.PATCH`

- `body: None | str | bytes`

  Body of the request. NeoCurl tries to parse it as `str`, on fails attempts to parse a `bytes`. Can be `None` to disable body.

- `timeout: None | int`

  Request timeout in milliseconds. Default is `100s`.

- `headers: None | Dict`

  Request headers. Parsed as a dictionary, can be `None`. Example:

  ```python
  headers = {
      "Content-Type": "binary/octet-stream",
      "User-Agent": "Neocurl/2.0.0-alpha.2",
  }
  ```

- `params: None | Dict`

  Query params. Parsed the same as `headers`, can be `None`.

### Example

```python
response = client.send(
    "https://httpbin.org/post",
    method = nc.POST,
    timeout = 10_000,
    headers = {
        "Content-Type": "binary/octet-stream",
        "User-Agent": "Neocurl/2.0.0-alpha.2",
    },
    params = {
        "id": "20438",
    },
)
```

## Get and Post

`nc.Client` has two more methods: `.get()` and `.post()`.
These dont need `method` specified.

## Return

Functions return `nc.Response`.

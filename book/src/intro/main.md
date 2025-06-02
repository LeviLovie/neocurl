# Introduction

Have you ever written a server and wanted to easely test it? I've always written a small request using curl. It might look something like this:

```bash
curl -X POST https://api.example.com/endpoint \
     -H "Content-Type: application/json" \
     -d '{"key1":"value1", "key2":"value2"}'
```

It works, but when you have multiple endpoints to test it will get out of hand pretty quickly.
It is also quite annoying to edit commands like this in the command line.

## Alternative

Before making this tool, I turned to [Paw](https://paw.cloud/) (Oh, yeah, its "RapidAPI" now).
But it is a quite heavy GUI app. For me, a nvim keyboard only user, it is so frustrating to have to use my mouse,
usually it on the other end of the table! The UI is also, of course its just me, confusing.

## About

At some point, after having to deal with yet one more curl syntax error after changing a request in a [Justfile](https://github.com/casey/just),
it was enough, I had to do something.

The idea I had was a simple CLI tool that runs [Lua](https://www.lua.org/) scripts and send requests defined in them.
My favorite programming language is [Rust](https://www.rust-lang.org/), so thats what I chose.
Fortunately, there are [amazing lua bindings for rust](https://github.com/mlua-rs/mlua).

Here is a preview of the script:

```lua
define({
    name = "get",
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

```bash
$ ncurl list
1: get

$ ncurl run get
Elapsed: 1843 ms
Status: 200 OK
Headers:
  content-type: application/json
  date: Sun, 01 Jun 2025 12:41:29 GMT
  access-control-allow-origin: *
  content-length: 263
  access-control-allow-credentials: true
  server: gunicorn/19.9.0
  connection: keep-alive
Body:
{
  "args": {},
  "headers": {
    "Accept": "application/json",
    "Host": "httpbin.org",
    "User-Agent": "Neocurl",
    "X-Amzn-Trace-Id": "Root=1-683c4a79-753e34797d9b9d4a7c020b49"
  },
  "origin": "184.22.77.52",
  "url": "https://httpbin.org/get"
}
```

It worked amazing! I iterated on the idea, eventually supporting a bunch of cool features
(look [here](https://github.com/LeviLovie/neocurl/blob/d268367d84363941a97e3eb95a1e92c2a086029d/README.md)).
Everything was wonderful, but the problem came with [msgpack](https://msgpack.org/index.html).
Lua was unable to properly handle msgpack.

After some thought, [Python](https://www.python.org/) was my choice.
I used [PyO₃](https://github.com/pyo3/pyo3) to run Python scripts from Rust.

Here is the same script, but in Python:

```python
@nc.define
def get(client):
    result = client.get(
        "https://httpbin.org/get",
        headers = {
            "User-Agent": "Neocurl",
            "Accept": "application/json"
        }
    )
    result.print()
```

```bash
$ cargo run run get
Response:
  Status: 200 200 OK
  Duration: 1699
  Headers:
    (date: Sun, 01 Jun 2025 12:51:23 GMT),
    (content-type: application/json),
    (content-length: 263),
    (connection: keep-alive),
    (server: gunicorn/19.9.0),
    (access-control-allow-origin: *),
    (access-control-allow-credentials: true)
  Body:
{
  "args": {},
  "headers": {
    "Accept": "application/json",
    "Host": "httpbin.org",
    "User-Agent": "Neocurl",
    "X-Amzn-Trace-Id": "Root=1-683c4ccb-10c14f0f5672dbd159a3b1fd"
  },
  "origin": "184.22.77.52",
  "url": "https://httpbin.org/get"
}
```

Now, msgpack works!

# Running

Run the `get` definition:

```bash
ncurl run get
```

```bash
13:20:38.496 INFO: Neocurl initialized
13:20:40.559 INFO get: Response status: 200 OK, finished in 2047.00ms
Response:
  Status: 200 200 OK
  Duration: 2047
  Headers:
    (date: Sun, 01 Jun 2025 13:20:40 GMT),
    (content-type: application/json),
    (content-length: 220),
    (connection: keep-alive),
    (server: gunicorn/19.9.0),
    (access-control-allow-origin: *),
    (access-control-allow-credentials: true)
  Body:
{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "X-Amzn-Trace-Id": "Root=1-683c53a7-0f57c1491bcba048080b1b12"
  },
  "origin": "184.22.77.52",
  "url": "https://httpbin.org/get"
}

Test results: 0/0
Call results: 1/0
13:20:40.559 INFO: Neocurl cleanup complete
```

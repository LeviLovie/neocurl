# Explanation

Lets take a look inside `ncurl.py`:

```python
import neocurl as nc

@nc.on_init
def main():
    if not nc.check_version("2.0.0-alpha.2"):
        nc.fatal(f"This version of neocurl is not compatible with this script: {nc.version()}")

    logger_config = nc.get_logger_config()
    if nc.env("LOG") == "DEBUG":
        logger_config.level = nc.LogLevel.Debug
    else:
        logger_config.level = nc.LogLevel.Info
    logger_config.datetime_format = "%H:%M:%S%.3f"
    logger_config.use_colors = True
    nc.set_logger_config(logger_config)

    nc.info("Neocurl initialized")

@nc.on_cleanup
def cleanup():
    nc.info("Neocurl cleanup complete")

@nc.define
def get(client):
    nc.debug("Sending GET request")

    response = client.get("https://httpbin.org/get")
    nc.info(f"Response status: {response.status}, finished in {response.duration:.2f}ms")

    assert response.status_code == 200, f"Expected status code 200, but got {response.status_code} ({response.status})"

    response.print()
```

This is not that scary as it look.

## Line by line

```python
import neocurl as nc
```

Module `neocurl` is provided by NeoCurl at runtime.

```python
@nc.on_init
def main():
```

This defines a function with `nc.on_init` decorator. NeoCurl will run this function before running any definitions.

```python
    if not nc.check_version("2.0.0-alpha.2"):
        nc.fatal(f"This version of neocurl is not compatible with this script: {nc.version()}")
```

This checks if the NeoCurl version is `2.0.0-alpha.2` (This might have beed updated since the book bas been written).
If the NeoCurl version does not match the version requested by the script, it fails with an error.
The function `nc.fatal(msg)` prints the message and exits the executable.

```python
    logger_config = nc.get_logger_config()
```

Gets the `logger_config` struct.

```python
    if nc.env("LOG") == "DEBUG":
        logger_config.level = nc.LogLevel.Debug
    else:
        logger_config.level = nc.LogLevel.Info
```

If there is a `LOG` env var and it is set to `DEBUG`, set the logger level to `Debug`, otherwise set `Info`.

```python
    logger_config.datetime_format = "%H:%M:%S%.3f"
    logger_config.use_colors = True
```

Sets the datatime format for logs and enabled colors.

```python
    nc.set_logger_config(logger_config)
```

Sets the logger config.

```python
    nc.info("Neocurl initialized")
```

Logs a message, saying NeoCurl has been initialized.

```python
@nc.on_cleanup
def cleanup():
    nc.info("Neocurl cleanup complete")
```

Defines a function with a `nc.on_cleanup` decorator. NeoCurl will run this after a definition is ran. The function logs a message about cleanup being successful.

```python
@nc.define
def get(client):
```

Defines a function `get` (The definition name) with a `nc.define` decorator. This will be ran when `ncurl run get` is executed.

```python
    response = client.get("https://httpbin.org/get")
    nc.info(f"Response status: {response.status}, finished in {response.duration:.2f}ms")
```

Send a `GET` request to `https://httpbin.org/get` and gets the response.
Logs the status and the duration.

```python
    assert response.status_code == 200, f"Expected status code 200, but got {response.status_code} ({response.status})"
```

Checks if the status code is 200, raises an error if it is not.

```python
    response.print()
```

Prints the response.

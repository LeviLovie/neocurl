import sys
import orjson
import neocurl as nc

@nc.on_init
def main():
    if not nc.check_version("2.0.0-indev"):
        nc.fatal(f"This version of neocurl is not compatible with this script: {nc.version()}")

    logger_config = nc.get_logger_config()
    if nc.env("LOG") == "DEBUG":
        logger_config.level = nc.LogLevel.Debug
    else:
        logger_config.level = nc.LogLevel.Info
    logger_config.datetime_format = "%H:%M:%S%.3f"
    logger_config.use_colors = True
    nc.set_logger_config(logger_config)
    nc.log(nc.LogLevel.Info, "Neocurl initialized")

@nc.define
def get(client):
    nc.debug("Sending GET request")

    response = client.send(nc.Request("https://httpbin.org/get"))
    nc.info(f"Response status: {response.status}, finished in {response.elapsed_seconds:.2f}s")

    if not nc.assert_t(response.status_code == 200):
        nc.error(f"Expected status code 200, but got {response.status_code} ({response.status})")

@nc.define
def post(client):
    nc.info("Sending POST request")

    request = nc.Request("https://httpbin.org/post")
    request.method = nc.Method.Post
    request.body = "Hello, World!"

    response = client.send(request)
    nc.info(f"Response status: {response.status}, finished in {response.elapsed_seconds:.2f}s")

    if not nc.assert_f(response.status_code != 200):
        nc.error(f"Expected status code 200, but got {response.status_code} ({response.status})")

    body = orjson.loads(response.body)
    nc.debug(f"Response body: {body}")

@nc.define
def fail(client):
    if not nc.assert_t(False):
        nc.error("This assertion should fail")

import sys
import orjson
import neocurl as nc

@nc.on_init
def main():
    if not nc.check_version("2.0.0"):
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

@nc.on_cleanup
def cleanup():
    nc.log(nc.LogLevel.Info, "Neocurl cleanup complete")

@nc.define
def get(client):
    nc.debug("Sending GET request")

    response = client.get("https://httpbin.org/get")
    nc.info(f"Response status: {response.status}, finished in {response.duration:.2f}ms")

    assert response.status_code == 200, f"Expected status code 200, but got {response.status_code} ({response.status})"

    response.print()

@nc.define
def post(client):
    nc.info("Sending POST request")

    response = client.send(
        "https://httpbin.org/post",
        method = nc.POST,
        body = "Hello, world!".encode(),
        headers = {
            "Content-Type": "binary/octet-stream",
            "User-Agent": "Neocurl/2.0.0-indev"
        },
        params = {
            "foo": "bar",
        }
    )
    nc.info(f"Response status: {response.status}, finished in {response.duration:.2f}ms")

    if not nc.assert_f(response.status_code != 200):
        nc.error(f"Expected status code 200, but got {response.status_code} ({response.status})")

    response.print()

    body = orjson.loads(response.body)
    nc.debug(f"Response body: {body}")

@nc.define
def get_async(client):
    nc.info("Sending asynchronous request")

    responses = client.send_async(
        "https://httpbin.org/get",
        timeout = 5000,
        threads = 256,
        amount = 64 * 64 * 4,
    )

    if not nc.assert_t(responses.amount() > 0):
        nc.fatal("No responses received")

    nc.info(f"Received {responses.amount()} responses")
    responses.print_stats(5, 3)

@nc.define
def new_client(_unused_client):
    nc.info("Creating a new client and sending a get request")

    nc.client().send("https://httpbin.org/get").print()

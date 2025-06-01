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

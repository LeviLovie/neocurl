import neocurl as nc

if not nc.check_version("1.3.1"):
    print("This version of neocurl is not compatible with this script:", nc.version())
    exit(1)

define("request", lambda: print("This is a request function from neocurl"))

define("request2", lambda: print("This is a second request function from neocurl"))


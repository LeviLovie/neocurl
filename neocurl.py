import sys
import neocurl as nc

@nc.on_init
def main():
    if not nc.check_version("1.3.1"):
        print("This version of neocurl is not compatible with this script:", nc.version())
        sys.exit(1)

def request():
    print("This is a request function from neocurl")
nc.define("request", request)

nc.define("request2", lambda: print("This is a second request function from neocurl"))


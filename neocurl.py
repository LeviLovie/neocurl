import sys
import neocurl as nc

@nc.on_init
def main():
    if not nc.check_version("1.3.1"):
        print("This version of neocurl is not compatible with this script:", nc.version())
        sys.exit(1)

@nc.define
def request():
    print("Executing request")

def log():
    print("FOO")

@nc.define
def log_request():
    log()

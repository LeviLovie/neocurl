import sys
import orjson
import neocurl as nc

@nc.on_init
def main():
    if not nc.check_version("2.0.0-indev"):
        print("This version of neocurl is not compatible with this script:", nc.version())
        sys.exit(1)

@nc.define
def get(client):
    response = client.send(nc.Request("https://httpbin.org/get"))
    print("Response status:", response.status)

@nc.define
def post(client):
    request = nc.Request("https://httpbin.org/post")
    request.method = nc.Method.Post
    request.body = "Hello, World!"

    response = client.send(request)
    print("Response status:", response.status)

    if response.status_code == 200:
        body = orjson.loads(response.body)
        print("Response body:", body)
        print("Response body headers:", body["headers"])

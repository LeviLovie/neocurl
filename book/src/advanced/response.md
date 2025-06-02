# Response

The return type of `nc.Client.send()` and similar methods is `nc.Response`, a struct defined in `neocurl` module.
It has no Python constructor and can only be retrieved from the client.

## Fields

- `status: str`

  Request status in a human readable form.

- `status_code: int`

  Request status as a num. E.g. `200`, `404`, `500`.

- `duration: int`

  Time elapsed to send request and recive a response in milliseconds.
  Does not include the time to form the request and time to parse the response.

- `body: None | str`

  Response body. Can be `None`.

- `headers: Dict`

  Response headers. A dictionary.

## Functions

- `print()`

  Prints information about the response in a human readable form.

# Response

## Sync

The return type of `nc.Client.send()` and similar methods is `nc.Response`, a struct defined in `neocurl` module. It has no Python constructor and can only be retrieved from the client.

### Fields

- `status: str`

  Request status in a human readable form.

- `status_code: int`

  Request status as a num. E.g. `200`, `404`, `500`.

- `duration: int`

  Time elapsed to send request and recive a response in milliseconds.
  Does not include the time to form the request and time to parse the response.

- `body: str`

  Response body.

- `body_raw: bytes`

  Response body as bytes.

- `headers: Dict`

  Response headers. A dictionary.

### Methods

- `print()`

  Prints information about the response in a human readable form.

## Async

The return type of async send functions is `nc.AsyncResponses`.

### Fields

- `responses: nc.Response[]`

  An array of responses.

### Methods

- `amount() -> int`

  Returns amount of responses.

- `print_nth(id)`

  Calls `print()` on nth element from `responses`.

  - `id: int`

    Id of response to print.

- `print_stats(chunk, cut off)`

  Prints responses statistics.

  - `chunk: int`

    Duration grouping chunk to use.

  - `cut_off: int`

    Cut off in percents. If amount is less than cut off, does not print the row.

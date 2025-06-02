# Async requests

To send async requests use `nc.send_async`.
It has two more keyword args:

- `amount: None | int`

  Amount of requests to send. Default is `1`.

- `threads: None | int`

  How many threads to use. Default is `1`.

Function returns an array of `nc.Response`.

## Rusty inner workings

NeoCurl uses [Tokio Runtime](https://docs.rs/tokio/latest/tokio/runtime/index.html) to schedule `amount` of tasks across `threads` concurent threads.

Total amount of requests ran is **not** `amount * threads`. Tasks are split equally across all threads.

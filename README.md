## Example Tokio Single Thread

The goal of this project is to experiment with [tokio] & [hyper] creating an asynchronous http
server that runs **in a single tread**. Also, is using an experimental hyper version
[`1.0.0-rc.3`](https://github.com/hyperium/hyper/blob/master/CHANGELOG.md#v100-rc3-2023-02-23).

## Run the server in release mode

```sh
cargo run --release
```

## Benchmarks

If [`wrk`](https://github.com/wg/wrk) is available you can test the performance of the server like
this:

```sh
# threads: 1
# open connections: 1
# run for 10s
wrk -t1 -c1 -d10s http://127.0.0.1:3000
```

There is also a plain nodejs http server available in `./node/server.js` you can use to compare the
performance between the two.

```bash
# start the server
node node/server.js --port 4032

# run the benchmark
wrk -t1 -c1 -d10s http://127.0.0.1:4032
```

## Dev mode

Install [cargo-watch]:

```sh
cargo install cargo-watch
```

Run the server in dev mode with tracing in debug level:

```sh
RUST_LOG=example_tokio_single_thread=debug cargo watch -w src -x run
```

**Next Goals**

-   integrate some benchmarking tools
-   support https
-   support http2
-   integrate rust docs generation

## Tokio Features

All main features are listed and enabled in the `Cargo.toml` file. These can be enabled at your
discretion.

Checkout the [tokio features] docs.

[tokio features]: https://docs.rs/crate/tokio/latest/features

## Ecosystem

| Crate               | Description                                                                       |
| ------------------- | --------------------------------------------------------------------------------- |
| [axum]              | -                                                                                 |
| [bytes]             | -                                                                                 |
| [hyper]             | ([examples](https://github.com/hyperium/hyper/tree/master/examples))              |
| [http-body]         | Asynchronous HTTP request or response body                                        |
| [http-body-util]    | Utilities for [http-body]                                                         |
| [log]               | A lightweight logging facade.                                                     |
| [env_logger]        | A logger configured via env vars, to use with the logging facade exposed by [log] |
| [pretty_env_logger] | Writes to standard error with nice colored output for log levels.                 |
| [serde_json]        | -                                                                                 |
| [tokio-macros]      | -                                                                                 |
| [tokio]             | -                                                                                 |
| [tower]             |                                                                                   |
| [tower-http]        |                                                                                   |
| [url]               | implementation of the URL Standard for the Rust programming language.             |

[hyper]: https://docs.rs/hyper
[http-body-util]: https://docs.rs/http-body-util
[bytes]: https://docs.rs/bytes
[tokio]: https://docs.rs/tokio
[axum]: https://docs.rs/axum
[log]: https://docs.rs/log
[env_logger]: https://docs.rs/env_logger
[pretty_env_logger]: https://docs.rs/pretty_env_logger
[tokio-macros]: https://docs.rs/tokio-macros
[http-body]: https://docs.rs/http-body
[url]: https://docs.rs/url
[serde_json]: https://docs.rs/serde_json
[cargo-watch]: https://crates.io/crates/cargo-watch
[tower]: https://docs.rs/tower/latest/tower/
[tower-http]: https://docs.rs/tower-http/latest/tower_http

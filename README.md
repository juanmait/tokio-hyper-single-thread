# Example Tokio Single Thread

The goal of this project is to experiment with [tokio] & [hyper] creating an asynchronous http
server that runs **in a single tread**. Also, is using an experimental hyper version
[`1.0.0-rc.3`](https://github.com/hyperium/hyper/blob/master/CHANGELOG.md#v100-rc3-2023-02-23).

## Run in Release Mode

Run the server in `--release` mode:

```sh
cargo run --release
```

## Development

Run the server in dev mode with tracing in debug level:

```sh
RUST_LOG=example_tokio_single_thread=debug cargo run
```

### Watch mode

Install [cargo-watch]:

```sh
cargo install cargo-watch
```

Run the server in dev mode with tracing in debug level:

```sh
RUST_LOG=example_tokio_single_thread=debug cargo watch -w src -x run
```

## Benchmarks

If [`wrk`](https://github.com/wg/wrk) is available you can test the performance of the server like
this:

NOTE: **First the rust server in `--release` mode for performance tests**

```sh
# threads: 1
# open connections: 1
# run for 10s
wrk -t1 -c1 -d10s http://127.0.0.1:3000
```

There is also a plain nodejs http server available in `./node/server.js`. You can use it to compare
the performance between the two.

```bash
# start the server
node node/server.js --port 4032

# run the benchmark
wrk -t1 -c1 -d10s http://127.0.0.1:4032
```

**Next Goals**

-   integrate more benchmarking tools
-   support https & http2
-   generate [rustdoc] documentation



## Ecosystem

| Crate               | Description                                                                       |
| ------------------- | --------------------------------------------------------------------------------- |
| [bytes]             | -                                                                                 |
| [env_logger]        | A logger configured via env vars, to use with the logging facade exposed by [log] |
| [hyper]             | ([examples](https://github.com/hyperium/hyper/tree/master/examples))              |
| [http-body]         | Asynchronous HTTP request or response body                                        |
| [http-body-util]    | Utilities for [http-body]                                                         |
| [log]               | A lightweight logging facade.                                                     |
| [pretty_env_logger] | Writes to standard error with nice colored output for log levels.                 |
| [tokio-macros]      | -                                                                                 |
| [tokio]             | -                                                                                 |
| [tower-http]        |                                                                                   |
| [url]               | implementation of the URL Standard for the Rust programming language.             |

[bytes]: https://docs.rs/bytes
[cargo-watch]: https://crates.io/crates/cargo-watch
[env_logger]: https://docs.rs/env_logger
[http-body]: https://docs.rs/http-body
[http-body-util]: https://docs.rs/http-body-util
[hyper]: https://docs.rs/hyper
[log]: https://docs.rs/log
[pretty_env_logger]: https://docs.rs/pretty_env_logger
[rustdoc]: https://doc.rust-lang.org/rustdoc/index.html
[tokio]: https://docs.rs/tokio
[tokio features]: https://docs.rs/crate/tokio/latest/features
[tokio-macros]: https://docs.rs/tokio-macros
[url]: https://docs.rs/url

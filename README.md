## Hyper Server

```sh
RUST_LOG=tokio_project=debug cargo watch -x run
```

## Tokio Features

All main features are listed and enabled in the `Cargo.toml` file. These can be enabled at your
discretion.

```toml
'fs',                   # Async file system access
'io-util',              # enable tokio_util::io
'io-std',               # enable tokio::io
'macros',               # Tokio macros
'net',                  # TCP/UDP/Unix bindings
'parking_lot',          # enable parkin_lot crate
'process',              # spawn child processes
'rt',                   # single tread scheduler
'rt-multi-thread',      # multi tread scheduler
'signal',               # handle system signals
'sync',                 # enable tokio::sync
'time',                 # enable tokio::time
'tracing',              # enable tracing crate
'mio',                  # low-level I/O features
```

Checkout the [tokio features] docs.

[tokio features]: https://docs.rs/crate/tokio/latest/features

## Ecosystem

| Crate               | Description                                                                       |
| ------------------- | --------------------------------------------------------------------------------- |
| [axum]              | -                                                                                 |
| [bytes]             | -                                                                                 |
| [hyper]             | -                                                                                 |
| [http-body]         | Asynchronous HTTP request or response body                                        |
| [http-body-util]    | Utilities for [http-body]                                                         |
| [log]               | A lightweight logging facade.                                                     |
| [env_logger]        | A logger configured via env vars, to use with the logging facade exposed by [log] |
| [pretty_env_logger] | Writes to standard error with nice colored output for log levels.                 |
| [serde_json]        | -                                                                                 |
| [tokio-macros]      | -                                                                                 |
| [tokio]             | -                                                                                 |
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

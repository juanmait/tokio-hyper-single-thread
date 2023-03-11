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

[bytes], [hyper], [http-body-util], [tokio], [axum], [pretty_env_logger], [tokio-macros], [env_logger]

[hyper]: https://docs.rs/hyper
[http-body-util]: https://docs.rs/http-body-util
[bytes]: https://docs.rs/bytes
[tokio]: https://docs.rs/tokio
[axum]: https://docs.rs/axum
[pretty_env_logger]: https://docs.rs/pretty_env_logger/latest/pretty_env_logger/
[tokio-macros]: https://docs.rs/tokio-macros
[env_logger]: https://docs.rs/env_logger

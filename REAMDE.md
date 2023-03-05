# Tokio Project

A starting point for a [tokio.rs] project. Check out the [crate's documentation] for more.

[tokio docs]: https://docs.rs/tokio/latest/tokio/
[crate's documentation]: https://docs.rs/tokio/latest/tokio/◊
[tokio.rs]: https://docs.rs/crate/tokio/latest

## Features

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

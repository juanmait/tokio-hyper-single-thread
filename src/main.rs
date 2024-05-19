//! Run with
//!
//! ```not_rust
//! RUST_LOG=example_tokio_single_thread=debug cargo watch -x run
//!
//! curl http://localhost:3000
//!
//! # JSON Post request
//! curl -X POST http://localhost:3000 -H "Content-Type: application/json" -d '{"Id": 79, "status": 3}'
//! ```
//!

use hyper::server::conn;
use hyper_util::rt::tokio::TokioIo;
use std::cell::Cell;
use std::net::SocketAddr;
use std::rc::Rc;
use tokio::net::TcpListener;

mod body;
mod service;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    // Build an async tokio runtime that runs everything on the current thread.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed building single thread runtime");

    // combine it with a `LocalSet, which means it can spawn `!Send` futures.
    let local_set = tokio::task::LocalSet::new();

    local_set.block_on(&rt, run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    let listener = TcpListener::bind(addr).await?;
    log::info!("Listening on http://{}", addr);

    // Using a !Send request counter is fine on 1 thread...
    let counter = Rc::new(Cell::new(0));

    loop {
        counter.set(counter.get() + 1);
        // non keep-alive connections will result in a new spawn
        // whereas keep-alive connections will reuse previous spawned services
        let (stream, _remote_addr) = listener.accept().await?;

        log::debug!("new stream accepted");

        // For each connection, clone the counter to use in our service...
        let counter_for_service = counter.clone();
        // We need another copy to report errors
        let counter_for_err_report = counter.clone();

        let mut connection = conn::http1::Builder::new();

        let join = tokio::task::spawn_local(async move {
            log::debug!("spawned new local task");

            connection
                .keep_alive(false)
                .serve_connection(
                    // Since `serve_connection` need something that implements hyper's IO traits Read & Write, and
                    // our `stream` is a tokio TcpStream that instead implements tokio's IO traits AsyncRead & AsyncWrite,
                    // we use the hyper utility struct `TokioIo` to act as a bridge between the two different IO implementations.
                    //
                    // `TokioIo` implements the needed  hyper's IO traits by calling the tokio's IO implementation internally.
                    // https://docs.rs/hyper/latest/hyper/server/conn/http1/struct.Builder.html#method.serve_connection
                    TokioIo::new(stream),
                    // hyper service
                    service::RouterService {
                        counter: counter_for_service,
                    },
                )
                .await
        });

        match join.await {
            Ok(_) => log::info!("Request number #{} completed OK.", counter.clone().get()),
            Err(e) => log::error!(
                "Something went wrong within a main local async task #{}: {:?}",
                counter_for_err_report.get(),
                e
            ),
        }
    }
}

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
        .expect("build runtime");

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
        // non keep-alive connections will result in a new spawn
        // whereas keep-alive connections will reuse previous spawned services
        let (stream, _) = listener.accept().await?;

        log::debug!("new stream accepted");

        // For each connection, clone the counter to use in our service...
        let cnt = counter.clone();
        let cnt1 = counter.clone();
        let service = service::Svc { counter: cnt };

        let result = tokio::task::spawn_local(async move {
            log::debug!("new local task spawn");

            let connection = conn::http1::Builder::new();

            // https://docs.rs/hyper/1.0.0-rc.3/hyper/server/conn/http1/struct.Builder.html#method.serve_connection
            let conn_result = connection.serve_connection(stream, service).await;

            if let Err(err) = conn_result {
                eprintln!("Error serving connection ({:?}): {:?}", cnt1, err);
                panic!();
            }
        })
        .await;

        match result {
            Ok(_) => println!("all went well: {:?}", counter.clone()),
            Err(e) => println!("Something went wrong serving a request. {:?}", e),
        }
    }
}

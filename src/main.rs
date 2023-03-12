//! Run with
//!
//! ```not_rust
//! RUST_LOG=tokio_project=debug cargo watch -x run
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

use hyper::body::{Body as HttpBody, Bytes, Frame};
use hyper::service::service_fn;
use hyper::{Error, Response};
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

mod service;

struct Body {
    // Our Body type is !Send and !Sync:
    _marker: PhantomData<*const ()>,
    data: Option<Bytes>,
}

impl From<String> for Body {
    fn from(a: String) -> Self {
        Body {
            _marker: PhantomData,
            data: Some(a.into()),
        }
    }
}

impl HttpBody for Body {
    type Data = Bytes;
    type Error = Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        Poll::Ready(self.get_mut().data.take().map(|d| Ok(Frame::data(d))))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    // Configure a runtime that runs everything on the current thread
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build runtime");

    // Combine it with a `LocalSet,  which means it can spawn !Send futures...
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, run())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

    // Using a !Send request counter is fine on 1 thread...
    let counter = Rc::new(Cell::new(0));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        // For each connection, clone the counter to use in our service...
        let cnt = counter.clone();

        // let service = service_fn(move |_| {
        //     let prev = cnt.get();
        //     cnt.set(prev + 1);
        //     let value = cnt.get();
        //     async move { Ok::<_, Error>(Response::new(Body::from(format!("Request #{}", value)))) }
        // });

        tokio::task::spawn_local(async move {
            if let Err(err) = conn::http1::Builder::new()
                .serve_connection(stream, service::Svc { counter: cnt })
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

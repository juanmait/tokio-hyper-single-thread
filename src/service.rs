use bytes::Bytes;
use http_body_util::Full;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

type FullBytes = Response<Full<Bytes>>;
pub struct Svc {
    pub counter: Rc<Cell<i32>>,
}

impl Svc {
    const NOT_FOUND: &str = "NOT_FOUND";

    fn response_full_bytes<T: Into<String>>(s: T) -> Result<FullBytes, hyper::Error> {
        Ok(Response::builder()
            .body(Full::new(Bytes::from(s.into())))
            .unwrap())
    }

    fn not_found() -> Result<FullBytes, hyper::Error> {
        Self::response_full_bytes(Self::NOT_FOUND)
    }

    fn handler(&mut self, req: Request<IncomingBody>) -> Result<FullBytes, hyper::Error> {
        let req_path = req.uri().path();
        let res = match req_path {
            "/" => Self::response_full_bytes(format!("home! counter = {:?}", self.counter)),
            _ => {
                // Return the 404 Not Found for other routes, and don't increment counter.
                return Self::not_found();
            }
        };

        if req_path != "/favicon.ico" {
            let prev = self.counter.get();
            self.counter.set(prev + 1);
            log::info!("Increasing counter to: {}", self.counter.get());
        }

        res
    }
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = FullBytes;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request<IncomingBody>) -> Self::Future {
        let res = self.handler(req);
        Box::pin(async { res })
    }
}

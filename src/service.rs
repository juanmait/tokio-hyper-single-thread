use bytes::Bytes;
use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::body::Body;

type FullBody = Response<Body>;
pub struct Svc {
    pub counter: Rc<Cell<i32>>,
}

impl Svc {
    const NOT_FOUND: &str = "NOT_FOUND";

    fn response_full_bytes<T>(s: T) -> Result<FullBody, hyper::Error>
    where
        T: Into<Bytes>,
    {
        let b = Body::from(s);
        let res = Response::new(b);
        Ok(res)
    }

    fn not_found() -> Result<FullBody, hyper::Error> {
        Self::response_full_bytes(Self::NOT_FOUND)
    }

    fn handler(&mut self, req: Request<IncomingBody>) -> Result<FullBody, hyper::Error> {
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
    type Response = FullBody;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request<IncomingBody>) -> Self::Future {
        let res = self.handler(req);
        Box::pin(async { res })
    }
}

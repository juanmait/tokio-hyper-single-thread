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

    fn mk_response(&self, s: String) -> Result<FullBytes, hyper::Error> {
        Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
    }

    fn not_found() -> Result<FullBytes, hyper::Error> {
        Ok(Response::builder()
            .body(Full::new(Bytes::from(Svc::NOT_FOUND)))
            .unwrap())
    }
}

impl Service<Request<IncomingBody>> for Svc {
    type Response = FullBytes;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request<IncomingBody>) -> Self::Future {
        let res = match req.uri().path() {
            "/" => self.mk_response(format!("home! counter = {:?}", self.counter)),
            "/posts" => self.mk_response(format!("posts, of course! counter = {:?}", self.counter)),
            "/authors" => self.mk_response(format!(
                "authors extraordinare! counter = {:?}",
                self.counter
            )),
            // Return the 404 Not Found for other routes, and don't increment counter.
            _ => {
                return Box::pin(async { Svc::not_found() });
            }
        };

        if req.uri().path() != "/favicon.ico" {
            let prev = self.counter.get();
            self.counter.set(prev + 1);

            log::info!("Increasing counter to: {}", self.counter.get());
        }

        Box::pin(async { res })
    }
}

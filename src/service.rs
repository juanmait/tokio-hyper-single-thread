use bytes::Bytes;
use hyper::service::Service;
use hyper::{body::Incoming, Request, Response};

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::body::Body;

type FullBody = Response<Body>;

/// Svc is a struct that implements the [hyper::service::Service] trait.
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

    fn home(&mut self, _: Request<Incoming>) -> Result<FullBody, hyper::Error> {
        Self::response_full_bytes(format!("home! counter = {:?}", self.counter))
    }

    fn route(&mut self, req: Request<Incoming>) -> Result<FullBody, hyper::Error> {
        let mut handled = false;
        let res = match req.uri().path() {
            "/" => {
                handled = true;
                self.home(req)
            }
            _ => {
                // Return the 404 Not Found for other routes, and don't increment counter.
                return Self::not_found();
            }
        };

        if handled {
            let prev = self.counter.get();
            self.counter.set(prev + 1);
            log::info!("Increasing counter to: {}", self.counter.get());
        }

        res
    }
}

impl Service<Request<Incoming>> for Svc {
    type Response = FullBody;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        let res = self.route(req);
        Box::pin(async { res })
    }
}

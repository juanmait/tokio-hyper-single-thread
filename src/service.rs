use bytes::Bytes;
use hyper::service::Service;
use hyper::{body::Incoming, Request, Response};

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::body::Body;

/// Svc implements [hyper::service::Service]
pub struct Svc {
    pub counter: Rc<Cell<i32>>,
}

impl Svc {
    const NOT_FOUND: &'static str = "NOT_FOUND";

    fn response_full_bytes<T>(into_bytes: T) -> Result<Response<Body>, hyper::Error>
    where
        T: Into<Bytes>,
    {
        let b = Body::from(into_bytes);
        let res = Response::new(b);
        Ok(res)
    }

    fn not_found() -> Result<Response<Body>, hyper::Error> {
        Self::response_full_bytes(Self::NOT_FOUND)
    }

    fn home(&self, _: Request<Incoming>) -> Result<Response<Body>, hyper::Error> {
        Self::response_full_bytes(format!("home! counter = {:?}", self.counter))
    }

    fn route(&self, req: Request<Incoming>) -> Result<Response<Body>, hyper::Error> {
        let mut handled = false;
        let path = req.uri().path();
        let res = match path {
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

/// Implement the Service trait for our Service.
///
/// > A Service is a function of a Request.
///
/// https://docs.rs/hyper/latest/hyper/service/trait.Service.html
impl Service<Request<Incoming>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let res = self.route(req);
        Box::pin(async { res })
    }
}

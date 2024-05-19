use bytes::Bytes;
use hyper::{body::Incoming, Request, Response};

use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::body::Body;

/// [hyper::service::Service] implementation for routing requests.
pub struct RouterService {
    pub counter: Rc<Cell<i32>>,
}

impl RouterService {
    const NOT_FOUND: &'static str = "NOT_FOUND";

    fn response_full_bytes<T>(into_bytes_impl: T) -> Result<Response<Body>, hyper::Error>
    where
        T: Into<Bytes>,
    {
        Ok(Response::new(Body::from(into_bytes_impl)))
    }

    fn home(&self, _: Request<Incoming>) -> Result<Response<Body>, hyper::Error> {
        Self::response_full_bytes(format!("home! counter = {:?}", self.counter))
    }

    fn route(&self, req: Request<Incoming>) -> Result<Response<Body>, hyper::Error> {
        let path = req.uri().path();
        let res = match path {
            "/" => self.home(req),
            _ => {
                // Return the 404 Not Found for other routes, and don't increment counter.
                return Self::response_full_bytes(Self::NOT_FOUND);
            }
        };

        res
    }
}

/// Implement the [Service] trait for our [RouterService].
///
/// > A Service is a function of a Request.
///
/// https://docs.rs/hyper/latest/hyper/service/trait.Service.html
impl hyper::service::Service<hyper::Request<hyper::body::Incoming>> for RouterService {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&self, req: hyper::Request<hyper::body::Incoming>) -> Self::Future {
        let res = self.route(req);
        Box::pin(async { res })
    }
}

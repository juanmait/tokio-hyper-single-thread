use bytes::Bytes;

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::Frame, Request, Response};
use serde_json::{Deserializer, StreamDeserializer, Value};
use std::convert::Infallible;

// An async function that consumes a request, does nothing with it and returns a
// response.
pub async fn echo(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let body = req.collect().await.unwrap().to_bytes();

    let json: Value = serde_json::from_slice(&body).unwrap();
    let json_str = json.to_string();
    let s = Bytes::from(json_str);

    let f = Full::new(s);

    Ok(Response::new(f))
}

pub async fn frame(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let incoming = req.into_body();
    let frame_stream = incoming.map_frame(|frame| {
        let frame = if let Ok(data) = frame.into_data() {
            log::debug!("Data: {:?}", data);

            data
        } else {
            Bytes::new()
        };

        Frame::data(frame)
    });

    Ok(Response::new(frame_stream.boxed()))
}

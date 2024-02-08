use std::{marker::PhantomData, pin::Pin};

use bytes::Bytes;
use hyper::body::Frame;
use hyper::Error;
use std::task::{Context, Poll};

/// see [examples/single_threaded.rs](https://github.com/hyperium/hyper/blob/afe278abe077cf85a29b6631b838cd335f50d30d/examples/single_threaded.rs#L31)
pub struct Body {
    // Force Body to be of type !Send and !Sync.
    // we use a phantom pointer `*const T`
    _marker: PhantomData<*const ()>,
    data: Option<Bytes>,
}

impl<T: Into<Bytes>> From<T> for Body {
    fn from(bytes: T) -> Self {
        Body {
            _marker: PhantomData,
            data: Some(bytes.into()),
        }
    }
}

impl hyper::body::Body for Body {
    type Data = Bytes;
    type Error = Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        log::debug!("Body::pool_frame() pooling data...");

        let data = self.get_mut().data.take();

        let frame = data.map(|d| {
            let frame = Frame::data(d);
            log::debug!(
                "Frame is_data/is_trailers: {}/{}",
                frame.is_data(),
                frame.is_trailers()
            );

            Ok(frame)
        });

        Poll::Ready(frame)
    }
}

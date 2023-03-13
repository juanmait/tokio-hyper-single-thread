use std::{marker::PhantomData, pin::Pin};

use bytes::Bytes;
use hyper::body::Frame;
use hyper::Error;
use std::task::{Context, Poll};

pub struct Body {
    // Our Body type is !Send and !Sync:
    _marker: PhantomData<*const ()>,
    data: Option<Bytes>,
}

impl<T: Into<Bytes>> From<T> for Body {
    fn from(a: T) -> Self {
        Body {
            _marker: PhantomData,
            data: Some(a.into()),
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
        log::debug!("Pooling data...");
        let data = self.get_mut().data.take();
        let frame = data.map(|d| Ok(Frame::data(d)));
        Poll::Ready(frame)
    }
}

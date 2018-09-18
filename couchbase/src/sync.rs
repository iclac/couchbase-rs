
//! Synchronization and Future/Stream abstractions.
use tokio::prelude::*;

use tokio_channel::mpsc;
use tokio_channel::oneshot;
use error::CouchbaseError;

pub struct CouchbaseFuture<T> {
    inner: oneshot::Receiver<Result<T, CouchbaseError>>,
}

impl<T> CouchbaseFuture<T> {
    pub fn new(rx: oneshot::Receiver<Result<T, CouchbaseError>>) -> Self {
        CouchbaseFuture { inner: rx }
    }
}

impl<T: Send + 'static> Future for CouchbaseFuture<T> {
    type Item = T;
    type Error = CouchbaseError;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.inner
            .poll()
            .expect("CouchbaseFuture shouldn't be canceled!")
        {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(Err(e)) => Err(e),
            Async::Ready(Ok(e)) => Ok(e.into()),
        }
    }
}

pub struct CouchbaseStream<T> {
    inner: mpsc::Receiver<Result<T, CouchbaseError>>,
}

impl<T> CouchbaseStream<T> {
    pub fn new(rx: mpsc::Receiver<Result<T, CouchbaseError>>) -> Self {
        CouchbaseStream { inner: rx }
    }
}

impl<T: Send + 'static> Stream for CouchbaseStream<T> {
    type Item = T;
    type Error = CouchbaseError;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        match self.inner
            .poll()
            .expect("CouchbaseStream shouldn't be canceled!")
        {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(Some(Ok(e))) => Ok(Async::Ready(Some(e))),
            Async::Ready(Some(Err(e))) => Err(e),
            Async::Ready(None) => Ok(Async::Ready(None))
        }
    }
}

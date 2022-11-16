mod create_description;
mod set_description;

pub use create_description::*;
pub use set_description::*;

use anyhow::Result;
use futures::task::AtomicWaker;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::*;

pub trait ObserverPromisifyExt {
    type Output;
    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<()>;
    fn wake(&self) -> Option<Result<Self::Output>>;
}

pub struct ObserverPromisify<T>
where
    T: ObserverPromisifyExt,
{
    waker: Arc<AtomicWaker>,
    begin: bool,
    ext: T,
}

impl<T> Future for ObserverPromisify<T>
where
    T: ObserverPromisifyExt + Unpin,
{
    type Output = anyhow::Result<T::Output>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut();
        this.waker.register(cx.waker());
        if !this.begin {
            match this.ext.handle(this.waker.clone()) {
                Err(e) => Poll::Ready(Err(e)),
                Ok(_) => {
                    this.begin = true;
                    Poll::Pending
                }
            }
        } else {
            this.ext
                .wake()
                .and_then(|r| Some(Poll::Ready(r)))
                .or_else(|| Some(Poll::Pending))
                .unwrap()
        }
    }
}

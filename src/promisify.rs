use std::{
    future::Future,
    pin::Pin,
    result::Result,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
    task::{Context, Poll},
    thread,
};

use futures::task::AtomicWaker;

pub trait PromisifyExt {
    type Err;
    type Output;

    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<(), Self::Err>;
    fn wake(&self) -> Option<Result<Self::Output, Self::Err>>;
}

pub struct Promisify<T>
where
    T: PromisifyExt,
{
    pub(crate) waker: Arc<AtomicWaker>,
    pub(crate) begin: bool,
    pub(crate) ext: T,
}

impl<T> Promisify<T>
where
    T: PromisifyExt,
{
    /// A wrapper for asynchronous tasks, used to handle asynchronous tasks that
    /// return results in a callback.
    ///
    /// ```no_run
    /// struct SimplePromisify;
    ///
    /// impl PromisifyExt for SimplePromisify {
    ///     type Err = ();
    ///     type Output = ();
    ///
    ///     fn handle(&self, waker: Arc<AtomicWaker>) -> Result<(), Self::Err> {
    ///         // Handle some asynchronous tasks...
    ///         waker.wake();
    ///         Ok(())
    ///     }
    ///
    ///     fn wake(&self) -> Option<Result<Self::Output, Self::Err>> {
    ///         // Handle the return value when woken up...
    ///         Some(Ok(()))
    ///     }
    /// }
    ///
    /// assert!(Promisify::new(SimplePromisify).await.is_ok());
    /// ```
    pub(crate) fn new(ext: T) -> Self {
        Self {
            waker: Arc::new(AtomicWaker::new()),
            begin: false,
            ext,
        }
    }
}

impl<T> Future for Promisify<T>
where
    T: PromisifyExt + Unpin,
{
    type Output = Result<T::Output, T::Err>;

    #[rustfmt::skip]
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let mut this = self.as_mut();
        this.waker.register(cx.waker());
        if !this.begin {
            match this.ext.handle(this.waker.clone()) {
                Err(e) => Poll::Ready(Err(e)),
                Ok(_) => {
                    this.begin = true;
                    Poll::Pending
                },
            }
        } else {
            this.ext
                .wake()
                .map(Poll::Ready)
                .unwrap_or(Poll::Pending)
        }
    }
}

pub struct SpawnBlocking<T, R> {
    handle: Option<thread::JoinHandle<()>>,
    waker: Arc<AtomicWaker>,
    process: Option<Box<T>>,
    ret: Arc<AtomicPtr<R>>,
}

impl<T, R> SpawnBlocking<T, R>
where
    T: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    /// Runs the provided function on an executor dedicated to blocking
    /// operations.
    ///
    /// ```no_run
    /// // This future never returns.
    /// SpawnBlocking::new(|| {
    ///     loop {
    ///         // an infinite loop.
    ///     }
    /// })
    /// .await;
    ///
    /// let ret = SpawnBlocking::new(|| "hello").await;
    ///
    /// assert_eq!(ret, "hello");
    /// ```
    pub(crate) fn new(func: T) -> Self {
        Self {
            ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
            waker: Arc::new(AtomicWaker::new()),
            process: Some(Box::new(func)),
            handle: None,
        }
    }
}

impl<T, R> Future for SpawnBlocking<T, R>
where
    T: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    type Output = R;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut();
        this.waker.register(cx.waker());
        match &this.handle {
            Some(handle) => {
                if handle.is_finished() {
                    let ret = this.ret.swap(std::ptr::null_mut(), Ordering::Relaxed);
                    return Poll::Ready(unsafe { *Box::from_raw(ret) });
                }
            }
            None => {
                let ret = this.ret.clone();
                let waker = this.waker.clone();
                let process = this.process.take();
                let _ = this.handle.insert(thread::spawn(move || {
                    if let Some(func) = process {
                        ret.store(Box::into_raw(Box::new(func())), Ordering::Relaxed);
                        waker.wake();
                    }
                }));
            }
        };

        Poll::Pending
    }
}

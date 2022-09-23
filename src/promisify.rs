use super::base::*;
use super::sys;
use super::rtc_peerconnection::*;
use super::rtc_session_description::*;
use anyhow::{anyhow, Result};
use futures::task::AtomicWaker;
use libc::*;
use std::convert::{TryFrom, TryInto};
use std::ffi::CString;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::task::*;

pub type CreateDescriptionFuture<'a> = ObserverPromisify<CreateDescriptionObserver<'a>>;
impl<'a> CreateDescriptionFuture<'a> {
    pub(crate) fn new(pc: &'a RawRTCPeerConnection, kind: CreateDescriptionKind) -> Self {
        Self {
            waker: Arc::new(AtomicWaker::new()),

            begin: false,
            ext: CreateDescriptionObserver {
                ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
                kind,
                pc,
            },
        }
    }
}

pub type SetDescriptionFuture<'a> = ObserverPromisify<SetDescriptionObserver<'a>>;
impl<'a> SetDescriptionFuture<'a> {
    pub(crate) fn new(
        pc: &'a RawRTCPeerConnection,
        desc: &'a RTCSessionDescription,
        kind: SetDescriptionKind,
    ) -> Self {
        Self {
            waker: Arc::new(AtomicWaker::new()),

            begin: false,
            ext: SetDescriptionObserver {
                ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
                desc,
                kind,
                pc,
            },
        }
    }
}

pub trait ObserverPromisifyExt {
    type Output;
    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<()>;
    fn wake(&self) -> Option<Result<Self::Output>>;
}

pub struct ObserverPromisify<T> {
    waker: Arc<AtomicWaker>,
    begin: bool,
    ext: T,
}

impl<T> Future for ObserverPromisify<T>
where
    T: ObserverPromisifyExt,
{
    type Output = anyhow::Result<T::Output>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_mut();
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
            match this.ext.wake() {
                Some(r) => Poll::Ready(r),
                None => Poll::Pending,
            }
        }
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum CreateDescriptionKind {
    Offer,
    Answer,
}

struct CreateDescriptionContext {
    callback: Box<dyn FnMut(Result<RTCSessionDescription>)>,
}

extern "C" fn create_description_callback(
    error: *const c_char,
    desc: *const RawRTCSessionDescription,
    ctx: *mut c_void,
) {
    let mut ctx = unsafe { Box::from_raw(ctx as *mut CreateDescriptionContext) };
    (ctx.callback)(
        from_raw_ptr(error)
            .map(|_| {
                cstr_to_string(error)
                    .map_err(|e| anyhow!(e.to_string()))
                    .and_then(|s| Err(anyhow!(s)))
            })
            .unwrap_or_else(|| RTCSessionDescription::try_from(unsafe { &*desc })),
    );
}

#[derive(PartialEq, PartialOrd)]
pub enum SetDescriptionKind {
    Local,
    Remote,
}

struct SetDescriptionContext {
    callback: Box<dyn FnMut(Result<()>)>,
}

extern "C" fn set_description_callback(error: *const c_char, ctx: *mut c_void) {
    let mut ctx = unsafe { Box::from_raw(ctx as *mut SetDescriptionContext) };
    (ctx.callback)(if error.is_null() {
        Ok(())
    } else {
        match unsafe { CString::from_raw(error as *mut c_char).into_string() } {
            Ok(e) => Err(anyhow!(e)),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    });
}

pub struct CreateDescriptionObserver<'a> {
    kind: CreateDescriptionKind,
    pc: &'a RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<RTCSessionDescription>>>,
}

impl<'a> ObserverPromisifyExt for CreateDescriptionObserver<'a> {
    type Output = RTCSessionDescription;

    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<()> {
        let ret = self.ret.clone();
        let ctx = Box::into_raw(Box::new(CreateDescriptionContext {
            callback: Box::new(move |res| {
                ret.store(Box::into_raw(Box::new(res)), Ordering::Relaxed);
                waker.wake();
            }),
        })) as *mut c_void;

        if self.kind == CreateDescriptionKind::Offer {
            unsafe { sys::rtc_create_offer(self.pc, create_description_callback, ctx) };
        } else {
            unsafe { sys::rtc_create_answer(self.pc, create_description_callback, ctx) };
        }

        Ok(())
    }

    fn wake(&self) -> Option<Result<Self::Output>> {
        let inner = self.ret.load(Ordering::Relaxed);
        if inner.is_null() {
            None
        } else {
            Some(Box::into_inner(unsafe { Box::from_raw(inner) }))
        }
    }
}

pub struct SetDescriptionObserver<'a> {
    kind: SetDescriptionKind,
    desc: &'a RTCSessionDescription,
    pc: &'a RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<()>>>,
}

impl<'a> ObserverPromisifyExt for SetDescriptionObserver<'a> {
    type Output = ();

    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<()> {
        let ret = self.ret.clone();
        let ctx = Box::into_raw(Box::new(SetDescriptionContext {
            callback: Box::new(move |res| {
                ret.store(Box::into_raw(Box::new(res)), Ordering::Relaxed);
                waker.wake();
            }),
        })) as *mut c_void;

        let desc: RawRTCSessionDescription = self.desc.try_into()?;
        let desc = Box::into_raw(Box::new(desc));

        if self.kind == SetDescriptionKind::Local {
            unsafe {
                sys::rtc_set_local_description(self.pc, desc, set_description_callback, ctx)
            };
        } else {
            unsafe {
                sys::rtc_set_remote_description(self.pc, desc, set_description_callback, ctx)
            };
        }

        Ok(())
    }

    fn wake(&self) -> Option<Result<Self::Output>> {
        let inner = self.ret.load(Ordering::Relaxed);
        if inner.is_null() {
            None
        } else {
            Some(Box::into_inner(unsafe { Box::from_raw(inner) }))
        }
    }
}

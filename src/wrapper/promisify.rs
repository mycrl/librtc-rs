use super::functions;
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

#[derive(PartialEq, PartialOrd)]
pub enum SessionDescriptionKind {
    Offer,
    Answer,
}

struct CreateSessionDescriptionContext {
    callback: Box<dyn FnMut(Result<RTCSessionDescription>)>,
}

pub struct CreateSessionDescriptionPromisify<'a> {
    waker: Arc<AtomicWaker>,
    kind: SessionDescriptionKind,
    peer: &'a RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<RTCSessionDescription>>>,
    begin: bool,
}

impl<'a> CreateSessionDescriptionPromisify<'a> {
    pub fn new(peer: &'a RawRTCPeerConnection, kind: SessionDescriptionKind) -> Self {
        Self {
            waker: Arc::new(AtomicWaker::new()),
            ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
            begin: false,
            peer,
            kind,
        }
    }
}

impl<'a> Future for CreateSessionDescriptionPromisify<'a> {
    type Output = anyhow::Result<RTCSessionDescription>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.as_ref().waker.register(cx.waker());

        if !self.as_ref().begin {
            let waker = self.as_ref().waker.clone();
            let desc = self.as_mut().ret.clone();
            let ctx = Box::into_raw(Box::new(CreateSessionDescriptionContext {
                callback: Box::new(move |res| {
                    desc.store(Box::into_raw(Box::new(res)), Ordering::Relaxed);
                    waker.wake();
                }),
            })) as *mut c_void;

            if self.as_ref().kind == SessionDescriptionKind::Offer {
                unsafe {
                    functions::rtc_create_offer(
                        self.as_ref().peer,
                        create_session_desc_callback,
                        ctx,
                    )
                };
            } else {
                unsafe {
                    functions::rtc_create_answer(
                        self.as_ref().peer,
                        create_session_desc_callback,
                        ctx,
                    )
                };
            }

            self.as_mut().begin = true;
            Poll::Pending
        } else {
            let inner = self.as_ref().ret.load(Ordering::Relaxed);
            if inner.is_null() {
                Poll::Pending
            } else {
                Poll::Ready(Box::into_inner(unsafe { Box::from_raw(inner) }))
            }
        }
    }
}

extern "C" fn create_session_desc_callback(
    error: *const c_char,
    desc: *const RawRTCSessionDescription,
    ctx: *mut c_void,
) {
    let mut ctx = unsafe { Box::from_raw(ctx as *mut CreateSessionDescriptionContext) };
    (ctx.callback)(if error.is_null() {
        RTCSessionDescription::try_from(unsafe { &*desc })
    } else {
        match unsafe { CString::from_raw(error as *mut c_char).into_string() } {
            Ok(e) => Err(anyhow!(e)),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    });
}

#[derive(PartialEq, PartialOrd)]
pub enum SetSessionDescriptionKind {
    Local,
    Remote,
}

struct SetSessionDescriptionContext {
    callback: Box<dyn FnMut(Result<()>)>,
}

pub struct SetSessionDescriptionPromisify<'a> {
    waker: Arc<AtomicWaker>,
    kind: SetSessionDescriptionKind,
    desc: &'a RTCSessionDescription,
    peer: &'a RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<()>>>,
    begin: bool,
}

impl<'a> SetSessionDescriptionPromisify<'a> {
    pub fn new(
        peer: &'a RawRTCPeerConnection,
        desc: &'a RTCSessionDescription,
        kind: SetSessionDescriptionKind,
    ) -> Self {
        Self {
            waker: Arc::new(AtomicWaker::new()),
            ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
            begin: false,
            peer,
            desc,
            kind,
        }
    }
}

impl<'a> Future for SetSessionDescriptionPromisify<'a> {
    type Output = anyhow::Result<()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.as_ref().waker.register(cx.waker());

        if !self.as_ref().begin {
            let waker = self.as_ref().waker.clone();
            let desc = self.as_mut().ret.clone();
            let ctx = Box::into_raw(Box::new(SetSessionDescriptionContext {
                callback: Box::new(move |res| {
                    desc.store(Box::into_raw(Box::new(res)), Ordering::Relaxed);
                    waker.wake();
                }),
            })) as *mut c_void;

            let desc: Result<RawRTCSessionDescription> = self.as_ref().desc.try_into();
            let desc = match desc {
                Ok(d) => Box::into_raw(Box::new(d)),
                Err(e) => return Poll::Ready(Err(e)),
            };

            if self.as_ref().kind == SetSessionDescriptionKind::Local {
                unsafe {
                    functions::rtc_set_local_description(
                        self.as_ref().peer,
                        desc,
                        set_session_desc_callback,
                        ctx,
                    )
                };
            } else {
                unsafe {
                    functions::rtc_set_remote_description(
                        self.as_ref().peer,
                        desc,
                        set_session_desc_callback,
                        ctx,
                    )
                };
            }

            self.as_mut().begin = true;
            Poll::Pending
        } else {
            let inner = self.as_ref().ret.load(Ordering::Relaxed);
            if inner.is_null() {
                Poll::Pending
            } else {
                Poll::Ready(Box::into_inner(unsafe { Box::from_raw(inner) }))
            }
        }
    }
}

extern "C" fn set_session_desc_callback(error: *const c_char, ctx: *mut c_void) {
    let mut ctx = unsafe { Box::from_raw(ctx as *mut SetSessionDescriptionContext) };
    (ctx.callback)(if error.is_null() {
        Ok(())
    } else {
        match unsafe { CString::from_raw(error as *mut c_char).into_string() } {
            Ok(e) => Err(anyhow!(e)),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    });
}

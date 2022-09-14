use super::raw;
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
    callback: Box<dyn FnMut(*const c_char, *const RawRTCSessionDescription)>,
}

pub struct CreateSessionDescriptionPromisify<'a> {
    waker: Arc<AtomicWaker>,
    kind: SessionDescriptionKind,
    peer: &'a RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<*mut RawRTCSessionDescription>>>,
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
            let ctx = Box::new(CreateSessionDescriptionContext {
                callback: Box::new(move |err, c_desc| {
                    desc.store(
                        Box::into_raw(Box::new(if err.is_null() {
                            Ok(c_desc as *mut RawRTCSessionDescription)
                        } else {
                            match unsafe { CString::from_raw(err as *mut c_char).into_string() } {
                                Ok(e) => Err(anyhow!(e)),
                                Err(e) => Err(anyhow!(e.to_string())),
                            }
                        })),
                        Ordering::Relaxed,
                    );
                    waker.wake();
                }),
            });

            if self.as_ref().kind == SessionDescriptionKind::Offer {
                unsafe {
                    raw::rtc_create_offer(
                        self.as_ref().peer,
                        create_session_desc_callback,
                        Box::into_raw(ctx) as *const c_void,
                    )
                };
            } else {
                unsafe {
                    raw::rtc_create_answer(
                        self.as_ref().peer,
                        create_session_desc_callback,
                        Box::into_raw(ctx) as *const c_void,
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
                Poll::Ready(match Box::into_inner(unsafe { Box::from_raw(inner) }) {
                    Ok(c_desc) => RTCSessionDescription::try_from(Box::into_inner(unsafe {
                        Box::from_raw(c_desc)
                    })),
                    Err(e) => Err(e),
                })
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
    (ctx.callback)(error, desc);
}

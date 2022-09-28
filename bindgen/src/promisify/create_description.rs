use super::{ObserverPromisify, ObserverPromisifyExt};
use crate::base::*;
use crate::rtc_peerconnection::*;
use crate::rtc_session_description::*;
use anyhow::{anyhow, Result};
use futures::task::AtomicWaker;
use libc::*;
use std::convert::TryFrom;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

#[link(name = "webrtc_sys")]
extern "C" {
    fn rtc_create_answer(
        pc: *const RawRTCPeerConnection,
        cb: extern "C" fn(*const c_char, *const RawRTCSessionDescription, *mut c_void),
        ctx: *mut c_void,
    );
    fn rtc_create_offer(
        pc: *const RawRTCPeerConnection,
        cb: extern "C" fn(*const c_char, *const RawRTCSessionDescription, *mut c_void),
        ctx: *mut c_void,
    );
}

pub type CreateDescriptionFuture<'a> = ObserverPromisify<CreateDescriptionObserver<'a>>;
impl<'a> CreateDescriptionFuture<'a> {
    pub(crate) fn new(pc: &'a RawRTCPeerConnection, kind: CreateDescriptionKind) -> Self {
        Self {
            begin: false,
            waker: Arc::new(AtomicWaker::new()),
            ext: CreateDescriptionObserver {
                ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
                kind,
                pc,
            },
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
            unsafe { rtc_create_offer(self.pc, create_description_callback, ctx) };
        } else {
            unsafe { rtc_create_answer(self.pc, create_description_callback, ctx) };
        }

        Ok(())
    }

    fn wake(&self) -> Option<Result<Self::Output>> {
        from_raw_mut_ptr(self.ret.swap(std::ptr::null_mut(), Ordering::Relaxed))
            .map(|ptr| unsafe { *Box::from_raw(ptr) })
    }
}

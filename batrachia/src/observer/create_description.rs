use futures::task::AtomicWaker;
use libc::*;
use anyhow::{
    anyhow,
    Result,
};

use super::{
    ObserverPromisify,
    ObserverPromisifyExt,
};

use crate::{
    base::*,
    rtc_peerconnection::*,
    rtc_session_description::*,
};

use std::{
    convert::TryFrom,
    sync::Arc,
};

use std::sync::atomic::{
    AtomicPtr,
    Ordering,
};

#[rustfmt::skip]
extern "C" {
    /// The create_answer() method on the RTCPeerConnection interface creates an
    /// SDP answer to an offer received from a remote peer during the
    /// offer/answer negotiation of a WebRTC connection. The answer contains
    /// information about any media already attached to the session, codecs and
    /// options supported by the browser, and any ICE candidates already
    /// gathered. The answer is delivered to the returned Future, and should
    /// then be sent to the source of the offer to continue the negotiation
    /// process.
    fn rtc_create_answer(
        pc: *const RawRTCPeerConnection,
        cb: extern "C" fn(*const c_char, *const RawRTCSessionDescription, *mut c_void),
        ctx: *mut c_void,
    );
    
    /// The create_offer() method of the RTCPeerConnection interface initiates
    /// the creation of an SDP offer for the purpose of starting a new WebRTC
    /// connection to a remote peer. The SDP offer includes information about
    /// any MediaStreamTrack objects already attached to the WebRTC session,
    /// codec, and options supported by the browser, and any candidates already
    /// gathered by the ICE agent, for the purpose of being sent over the
    /// signaling channel to a potential peer to request a connection or to
    /// update the configuration of an existing connection.
    fn rtc_create_offer(
        pc: *const RawRTCPeerConnection,
        cb: extern "C" fn(*const c_char, *const RawRTCSessionDescription, *mut c_void,),
        ctx: *mut c_void,
    );
}

#[derive(PartialEq, Eq, PartialOrd)]
pub(crate) enum CreateDescriptionKind {
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
    let mut ctx =
        unsafe { Box::from_raw(ctx as *mut CreateDescriptionContext) };
    (ctx.callback)(
        from_raw_ptr(error)
            .map(|_| {
                from_c_str(error)
                    .map_err(|e| anyhow!(e.to_string()))
                    .and_then(|s| Err(anyhow!(s)))
            })
            .unwrap_or_else(|| {
                RTCSessionDescription::try_from(unsafe { &*desc })
            }),
    );
}

pub struct CreateDescriptionObserver {
    kind: CreateDescriptionKind,
    pc: *const RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<RTCSessionDescription>>>,
}

unsafe impl Send for CreateDescriptionObserver {}
unsafe impl Sync for CreateDescriptionObserver {}

impl ObserverPromisifyExt for CreateDescriptionObserver {
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
            unsafe {
                rtc_create_offer(self.pc, create_description_callback, ctx)
            };
        } else {
            unsafe {
                rtc_create_answer(self.pc, create_description_callback, ctx)
            };
        }

        Ok(())
    }

    fn wake(&self) -> Option<Result<Self::Output>> {
        from_raw_mut_ptr(self.ret.swap(std::ptr::null_mut(), Ordering::Relaxed))
            .map(|ptr| unsafe { *Box::from_raw(ptr) })
    }
}

pub type CreateDescriptionFuture = ObserverPromisify<CreateDescriptionObserver>;
impl CreateDescriptionFuture {
    pub(crate) fn new(
        pc: *const RawRTCPeerConnection,
        kind: CreateDescriptionKind,
    ) -> Self {
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

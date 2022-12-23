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
    symbols::*,
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

#[derive(PartialEq, Eq, PartialOrd)]
pub(crate) enum CreateDescriptionKind {
    Offer,
    Answer,
}

struct CreateDescriptionContext {
    callback: Box<dyn FnMut(Result<RTCSessionDescription>)>,
}

#[no_mangle]
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
    pc:   *const RawRTCPeerConnection,
    ret:  Arc<AtomicPtr<Result<RTCSessionDescription>>>,
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
            ext:   CreateDescriptionObserver {
                ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
                kind,
                pc,
            },
        }
    }
}

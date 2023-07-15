use std::{
    ffi::{c_char, c_void},
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};

use anyhow::{anyhow, Result};
use futures::task::AtomicWaker;

use crate::{
    cstr::from_c_str, rtc_peerconnection::RawRTCPeerConnection,
    rtc_session_description::RawRTCSessionDescription, Promisify, PromisifyExt,
    RTCSessionDescription,
};

extern "C" {
    pub(crate) fn rtc_set_local_description(
        peer: *const crate::rtc_peerconnection::RawRTCPeerConnection,
        desc: *const crate::rtc_session_description::RawRTCSessionDescription,
        callback: extern "C" fn(*const c_char, *mut c_void),
        ctx: *mut c_void,
    );

    pub(crate) fn rtc_set_remote_description(
        peer: *const crate::rtc_peerconnection::RawRTCPeerConnection,
        desc: *const crate::rtc_session_description::RawRTCSessionDescription,
        callback: extern "C" fn(*const c_char, *mut c_void),
        ctx: *mut c_void,
    );
}

#[derive(PartialEq, Eq, PartialOrd)]
pub(crate) enum SetDescriptionKind {
    Local,
    Remote,
}

struct SetDescriptionContext {
    callback: Box<dyn FnMut(Result<()>)>,
}

#[no_mangle]
extern "C" fn set_description_callback(error: *const c_char, ctx: *mut c_void) {
    let mut ctx = unsafe { Box::from_raw(ctx as *mut SetDescriptionContext) };
    (ctx.callback)(
        unsafe { error.as_ref() }
            .map(|_| {
                from_c_str(error)
                    .map_err(|e| anyhow!(e.to_string()))
                    .and_then(|s| Err(anyhow!(s)))
            })
            .unwrap_or_else(|| Ok(())),
    );
}

pub struct SetDescriptionObserver<'a> {
    kind: SetDescriptionKind,
    desc: &'a RTCSessionDescription,
    pc: *const RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<()>>>,
}

unsafe impl Send for SetDescriptionObserver<'_> {}
unsafe impl Sync for SetDescriptionObserver<'_> {}

impl<'a> PromisifyExt for SetDescriptionObserver<'a> {
    type Err = anyhow::Error;
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
        if self.kind == SetDescriptionKind::Local {
            unsafe { rtc_set_local_description(self.pc, &desc, set_description_callback, ctx) };
        } else {
            unsafe { rtc_set_remote_description(self.pc, &desc, set_description_callback, ctx) };
        }

        Ok(())
    }

    fn wake(&self) -> Option<Result<Self::Output>> {
        unsafe {
            self.ret
                .swap(std::ptr::null_mut(), Ordering::Relaxed)
                .as_mut()
        }
        .map(|ptr| unsafe { *Box::from_raw(ptr) })
    }
}

pub type SetDescriptionFuture<'a> = Promisify<SetDescriptionObserver<'a>>;
impl<'a> SetDescriptionFuture<'a> {
    pub(crate) fn create(
        pc: *const RawRTCPeerConnection,
        desc: &'a RTCSessionDescription,
        kind: SetDescriptionKind,
    ) -> Self {
        Promisify::new(SetDescriptionObserver {
            ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
            desc,
            kind,
            pc,
        })
    }
}

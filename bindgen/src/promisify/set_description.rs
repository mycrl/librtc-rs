use super::{ObserverPromisify, ObserverPromisifyExt};
use crate::base::*;
use crate::rtc_peerconnection::*;
use crate::rtc_session_description::*;
use crate::sys;
use anyhow::{anyhow, Result};
use futures::task::AtomicWaker;
use libc::*;
use std::convert::TryInto;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

pub type SetDescriptionFuture<'a> = ObserverPromisify<SetDescriptionObserver<'a>>;
impl<'a> SetDescriptionFuture<'a> {
    pub(crate) fn new(
        pc: &'a RawRTCPeerConnection,
        desc: &'a RTCSessionDescription,
        kind: SetDescriptionKind,
    ) -> Self {
        Self {
            begin: false,
            waker: Arc::new(AtomicWaker::new()),
            ext: SetDescriptionObserver {
                ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
                desc,
                kind,
                pc,
            },
        }
    }
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
    (ctx.callback)(
        from_raw_ptr(error)
            .map(|_| {
                cstr_to_string(error)
                    .map_err(|e| anyhow!(e.to_string()))
                    .and_then(|s| Err(anyhow!(s)))
            })
            .unwrap_or_else(|| Ok(())),
    );
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
            unsafe { sys::rtc_set_local_description(self.pc, desc, set_description_callback, ctx) };
            
        } else {
            unsafe {
                sys::rtc_set_remote_description(self.pc, desc, set_description_callback, ctx)
            };
        }

        unsafe { Box::from_raw(desc) };
        Ok(())
    }

    fn wake(&self) -> Option<Result<Self::Output>> {
        from_raw_mut_ptr(self.ret.swap(std::ptr::null_mut(), Ordering::Relaxed))
            .map(|ptr| unsafe { *Box::from_raw(ptr) })
    }
}

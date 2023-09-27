use std::{
    error::Error,
    ffi::{c_char, c_void},
    fmt,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};

use futures::task::AtomicWaker;

use crate::{
    cstr::{from_c_str, StringError},
    rtc_peerconnection::RawRTCPeerConnection,
    rtc_session_description::RawRTCSessionDescription,
    Promisify, PromisifyExt, RTCSessionDescription,
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

#[derive(Debug)]
pub enum SetDescriptionError {
    StringError(StringError),
    SetFailed(String),
}

impl Error for SetDescriptionError {}

impl fmt::Display for SetDescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub(crate) enum SetDescriptionKind {
    Local,
    Remote,
}

struct SetDescriptionContext {
    callback: Box<dyn FnMut(Result<(), SetDescriptionError>)>,
}

#[no_mangle]
extern "C" fn set_description_callback(error: *const c_char, ctx: *mut c_void) {
    let mut ctx = unsafe { Box::from_raw(ctx as *mut SetDescriptionContext) };
    (ctx.callback)(
        unsafe { error.as_ref() }
            .map(|_| {
                from_c_str(error)
                    .map_err(|e| SetDescriptionError::StringError(e))
                    .and_then(|s| Err(SetDescriptionError::SetFailed(s)))
            })
            .unwrap_or_else(|| Ok(())),
    );
}

pub struct SetDescriptionObserver<'a> {
    kind: SetDescriptionKind,
    desc: &'a RTCSessionDescription,
    pc: *const RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<(), SetDescriptionError>>>,
}

unsafe impl Send for SetDescriptionObserver<'_> {}
unsafe impl Sync for SetDescriptionObserver<'_> {}

impl<'a> PromisifyExt for SetDescriptionObserver<'a> {
    type Err = SetDescriptionError;
    type Output = ();

    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<(), Self::Err> {
        let ret = self.ret.clone();
        let ctx = Box::into_raw(Box::new(SetDescriptionContext {
            callback: Box::new(move |res| {
                ret.store(Box::into_raw(Box::new(res)), Ordering::Relaxed);
                waker.wake();
            }),
        })) as *mut c_void;

        let desc: RawRTCSessionDescription = self
            .desc
            .try_into()
            .map_err(|e| SetDescriptionError::StringError(e))?;
        if self.kind == SetDescriptionKind::Local {
            unsafe { rtc_set_local_description(self.pc, &desc, set_description_callback, ctx) };
        } else {
            unsafe { rtc_set_remote_description(self.pc, &desc, set_description_callback, ctx) };
        }

        Ok(())
    }

    fn wake(&self) -> Option<Result<Self::Output, Self::Err>> {
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

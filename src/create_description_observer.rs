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
    pub(crate) fn rtc_create_answer(
        pc: *const crate::rtc_peerconnection::RawRTCPeerConnection,
        cb: extern "C" fn(
            *const c_char,
            *const crate::rtc_session_description::RawRTCSessionDescription,
            *mut c_void,
        ),
        ctx: *mut c_void,
    );

    pub(crate) fn rtc_create_offer(
        pc: *const crate::rtc_peerconnection::RawRTCPeerConnection,
        cb: extern "C" fn(
            *const c_char,
            *const crate::rtc_session_description::RawRTCSessionDescription,
            *mut c_void,
        ),
        ctx: *mut c_void,
    );
}

#[derive(Debug)]
pub enum CreateDescriptionError {
    StringError(StringError),
    CreateFailed(String),
}

impl Error for CreateDescriptionError {}

impl fmt::Display for CreateDescriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub(crate) enum CreateDescriptionKind {
    Offer,
    Answer,
}

struct CreateDescriptionContext {
    callback: Box<dyn FnMut(Result<RTCSessionDescription, CreateDescriptionError>)>,
}

#[no_mangle]
extern "C" fn create_description_callback(
    error: *const c_char,
    desc: *const RawRTCSessionDescription,
    ctx: *mut c_void,
) {
    let mut ctx = unsafe { Box::from_raw(ctx as *mut CreateDescriptionContext) };
    (ctx.callback)(
        unsafe { error.as_ref() }
            .map(|_| {
                from_c_str(error)
                    .map_err(|e| CreateDescriptionError::StringError(e))
                    .and_then(|s| Err(CreateDescriptionError::CreateFailed(s)))
            })
            .unwrap_or_else(|| {
                RTCSessionDescription::try_from(unsafe { &*desc })
                    .map_err(|e| CreateDescriptionError::StringError(e))
            }),
    );
}

pub struct CreateDescriptionObserver {
    kind: CreateDescriptionKind,
    pc: *const RawRTCPeerConnection,
    ret: Arc<AtomicPtr<Result<RTCSessionDescription, CreateDescriptionError>>>,
}

unsafe impl Send for CreateDescriptionObserver {}
unsafe impl Sync for CreateDescriptionObserver {}

impl PromisifyExt for CreateDescriptionObserver {
    type Output = RTCSessionDescription;
    type Err = CreateDescriptionError;

    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<(), Self::Err> {
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

    fn wake(&self) -> Option<Result<Self::Output, Self::Err>> {
        unsafe {
            self.ret
                .swap(std::ptr::null_mut(), Ordering::Relaxed)
                .as_mut()
        }
        .map(|ptr| unsafe { *Box::from_raw(ptr) })
    }
}

pub type CreateDescriptionFuture = Promisify<CreateDescriptionObserver>;
impl CreateDescriptionFuture {
    pub(crate) fn create(pc: *const RawRTCPeerConnection, kind: CreateDescriptionKind) -> Self {
        Promisify::new(CreateDescriptionObserver {
            ret: Arc::new(AtomicPtr::new(std::ptr::null_mut())),
            kind,
            pc,
        })
    }
}

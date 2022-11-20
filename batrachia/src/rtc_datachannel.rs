use libc::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawRTCDataChannel {
    pub id: *const c_char,
    pub label: *const c_char,
}

#[derive(Debug, Clone)]
pub struct RTCDataChannel {}

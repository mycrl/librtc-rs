use libc::*;

#[repr(C)]
pub struct RTCDataChannel {
    pub id: *const c_char,
    pub label: *const c_char,
}

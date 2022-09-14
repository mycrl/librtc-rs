use libc::*;

#[repr(C)]
pub struct RTCIceCandidate {
    pub candidate: *const c_char,
    pub sdp_mid: *const c_char,
    pub sdp_mline_index: c_int,
}

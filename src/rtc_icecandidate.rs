use super::base::*;
use libc::*;
use std::convert::*;
use std::ffi::{CStr, CString};

#[repr(C)]
pub struct RawRTCIceCandidate {
    pub candidate: *const c_char,
    pub sdp_mid: *const c_char,
    pub sdp_mline_index: c_int,
}

impl Drop for RawRTCIceCandidate {
    fn drop(&mut self) {
        free_cstring(self.sdp_mid as *mut c_char);
        free_cstring(self.candidate as *mut c_char);
    }
}

pub struct RTCIceCandidate {
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_mline_index: u8,
}

impl TryInto<RawRTCIceCandidate> for &RTCIceCandidate {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<RawRTCIceCandidate, Self::Error> {
        Ok(RawRTCIceCandidate {
            sdp_mline_index: self.sdp_mline_index as c_int,
            sdp_mid: CString::new(self.sdp_mid.to_string())?.into_raw(),
            candidate: CString::new(self.candidate.to_string())?.into_raw(),
        })
    }
}

impl TryFrom<&RawRTCIceCandidate> for RTCIceCandidate {
    type Error = anyhow::Error;
    fn try_from(value: &RawRTCIceCandidate) -> Result<Self, Self::Error> {
        Ok(RTCIceCandidate {
            sdp_mline_index: value.sdp_mline_index as u8,
            sdp_mid: unsafe { CStr::from_ptr(value.sdp_mid).to_str()?.to_string() },
            candidate: unsafe { CStr::from_ptr(value.candidate).to_str()?.to_string() },
        })
    }
}

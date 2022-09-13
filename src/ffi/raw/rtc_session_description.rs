use anyhow::Result;
use libc::*;
use std::convert::{TryFrom, TryInto};
use std::ffi::CString;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum RtcSessionDescriptionType {
    Offer,
    PrAnswer,
    Answer,
    Rollback,
}

impl Default for RtcSessionDescriptionType {
    fn default() -> Self {
        Self::Offer
    }
}

#[repr(C)]
pub struct RawRTCSessionDescription {
    kind: RtcSessionDescriptionType,
    sdp: *const c_char,
}

impl RawRTCSessionDescription {
    pub fn into_raw(self) -> *const Self {
        Box::into_raw(Box::new(self))
    }

    pub fn from_raw(raw: *const Self) -> Box<Self> {
        unsafe { Box::from_raw(raw as *mut Self) }
    }
}

/// RTCSessionDescription
///
/// The RTCSessionDescription interface describes one end of a connection or
/// potential connection and how it's configured. Each RTCSessionDescription
/// consists of a description type indicating which part of the offer/answer
/// negotiation process it describes and of the SDP descriptor of the session.
#[derive(Clone, Debug, Default)]
pub struct RTCSessionDescription {
    pub kind: RtcSessionDescriptionType,
    pub sdp: String,
}

impl TryInto<RawRTCSessionDescription> for RTCSessionDescription {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<RawRTCSessionDescription, Self::Error> {
        Ok(RawRTCSessionDescription {
            kind: self.kind,
            sdp: CString::new(self.sdp)?.into_raw(),
        })
    }
}

impl TryFrom<RawRTCSessionDescription> for RTCSessionDescription {
    type Error = anyhow::Error;
    fn try_from(value: RawRTCSessionDescription) -> Result<Self, Self::Error> {
        Ok(RTCSessionDescription {
            kind: value.kind,
            sdp: unsafe {
                CString::from_raw(value.sdp as *mut c_char)
                    .as_c_str()
                    .to_str()?
                    .to_string()
            },
        })
    }
}

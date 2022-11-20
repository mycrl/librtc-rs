use super::base::*;
use anyhow::Result;
use libc::*;
use std::convert::{TryFrom, TryInto};

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum RTCSessionDescriptionType {
    /// The session description object describes the initial proposal in an
    /// offer/answer exchange. The session negotiation process begins with an offer
    /// being sent from the caller to the callee.
    Offer,
    /// Description must be treated as an SDP answer, but not a final answer.
    PrAnswer,
    /// The SDP contained in the sdp property is the definitive choice in the
    /// exchange. In other words, this session description describes the agreed-upon
    /// configuration, and is being sent to finalize negotiation.
    Answer,
    /// This special type with an empty session description is used to
    /// roll back to the previous stable state.
    Rollback,
}

impl Default for RTCSessionDescriptionType {
    fn default() -> Self {
        Self::Offer
    }
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct RawRTCSessionDescription {
    r#type: RTCSessionDescriptionType,
    sdp: *const c_char,
}

impl Drop for RawRTCSessionDescription {
    fn drop(&mut self) {
        free_cstring(self.sdp as *mut c_char);
    }
}

/// RTCSessionDescription
///
/// The RTCSessionDescription interface describes one end of a connection or
/// potential connection and how it's configured. Each RTCSessionDescription
/// consists of a description type indicating which part of the offer/answer
/// negotiation process it describes and of the SDP descriptor of the session.
#[derive(Clone, Debug)]
pub struct RTCSessionDescription {
    pub kind: RTCSessionDescriptionType,
    /// A string containing the SDP describing the session.
    pub sdp: String,
}

unsafe impl Send for RTCSessionDescription {}
unsafe impl Sync for RTCSessionDescription {}

impl TryInto<RawRTCSessionDescription> for &RTCSessionDescription {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<RawRTCSessionDescription, Self::Error> {
        Ok(RawRTCSessionDescription {
            sdp: to_c_str(&self.sdp)?,
            r#type: self.kind,
        })
    }
}

impl TryFrom<&RawRTCSessionDescription> for RTCSessionDescription {
    type Error = anyhow::Error;
    fn try_from(value: &RawRTCSessionDescription) -> Result<Self, Self::Error> {
        Ok(RTCSessionDescription {
            sdp: from_c_str(value.sdp)?,
            kind: value.r#type,
        })
    }
}

use std::{
    convert::{TryFrom, TryInto},
    ffi::c_char,
};

use serde::{Deserialize, Serialize};

use crate::cstr::{free_cstring, from_c_str, to_c_str, StringError};

/// An enum describing the session description's type.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RTCSessionDescriptionType {
    /// The session description object describes the initial proposal in an
    /// offer/answer exchange. The session negotiation process begins with an
    /// offer being sent from the caller to the callee.
    Offer,
    /// Description must be treated as an SDP answer, but not a final answer.
    PrAnswer,
    /// The SDP contained in the sdp property is the definitive choice in the
    /// exchange. In other words, this session description describes the
    /// agreed-upon configuration, and is being sent to finalize
    /// negotiation.
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
#[derive(Debug)]
pub(crate) struct RawRTCSessionDescription {
    r#type: RTCSessionDescriptionType,
    sdp: *const c_char,
}

impl Drop for RawRTCSessionDescription {
    fn drop(&mut self) {
        free_cstring(self.sdp as *mut c_char);
    }
}

/// The RTCSessionDescription interface describes one end of a connection or
/// potential connection and how it's configured.
///
/// Each RTCSessionDescription consists of a description type indicating which
/// part of the offer/answer negotiation process it describes and of the SDP
/// descriptor of the session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RTCSessionDescription {
    #[serde(rename = "type")]
    pub kind: RTCSessionDescriptionType,
    /// A string containing the SDP describing the session.
    pub sdp: String,
}

unsafe impl Send for RTCSessionDescription {}
unsafe impl Sync for RTCSessionDescription {}

impl TryInto<RawRTCSessionDescription> for &RTCSessionDescription {
    type Error = StringError;

    fn try_into(self) -> Result<RawRTCSessionDescription, Self::Error> {
        Ok(RawRTCSessionDescription {
            sdp: to_c_str(&self.sdp)?,
            r#type: self.kind,
        })
    }
}

impl TryFrom<&RawRTCSessionDescription> for RTCSessionDescription {
    type Error = StringError;

    fn try_from(value: &RawRTCSessionDescription) -> Result<Self, Self::Error> {
        Ok(RTCSessionDescription {
            sdp: from_c_str(value.sdp)?,
            kind: value.r#type,
        })
    }
}

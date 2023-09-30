use std::ffi::{c_char, c_int};

use serde::{Deserialize, Serialize};

use crate::cstr::{free_cstring, from_c_str, to_c_str, StringError};

#[repr(C)]
pub(crate) struct RawRTCIceCandidate {
    pub candidate: *const c_char,
    pub sdp_mid: *const c_char,
    pub sdp_mline_index: c_int,
}

impl Drop for RawRTCIceCandidate {
    fn drop(&mut self) {
        free_cstring(self.sdp_mid.cast_mut());
        free_cstring(self.candidate.cast_mut());
    }
}

/// Indicates a candidate Interactive Connection Establishment
/// (ICE) configuration.
///
/// The RTCIceCandidate interface¡ªpart of the WebRTC API¡ªrepresents a
/// candidate Interactive Connectivity Establishment (ICE) configuration which
/// may be used to establish an RTCPeerConnection.
///
/// An ICE candidate describes the protocols and routing needed for WebRTC to be
/// able to communicate with a remote device. When starting a WebRTC peer
/// connection, typically a number of candidates are proposed by each end of the
/// connection, until they mutually agree upon one which describes the
/// connection they decide will be best. WebRTC then uses that candidate's
/// details to initiate the connection.
///
/// For details on how the ICE process works, see Lifetime of a WebRTC session.
/// The article WebRTC connectivity provides additional useful details.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RTCIceCandidate {
    /// A string describing the properties of the candidate, taken directly
    /// from the SDP attribute "candidate". The candidate string specifies
    /// the network connectivity information for the candidate. If the
    /// candidate is an empty string (""), the end of the candidate list
    /// has been reached; this candidate is known as the
    /// "end-of-candidates" marker.
    pub candidate: String,
    /// A string containing the identification tag of the media stream with
    /// which the candidate is associated, or null if there is no
    /// associated media stream. The default is null.
    pub sdp_mid: String,
    /// TA number property containing the zero-based index of the m-line with
    /// which Tthe candidate is associated, within the SDP of the media
    /// description, or Tnull if no such associated exists. The default is
    /// null.
    pub sdp_mline_index: u8,
}

impl TryInto<RawRTCIceCandidate> for &RTCIceCandidate {
    type Error = StringError;

    fn try_into(self) -> Result<RawRTCIceCandidate, Self::Error> {
        Ok(RawRTCIceCandidate {
            sdp_mline_index: self.sdp_mline_index as c_int,
            sdp_mid: to_c_str(&self.sdp_mid)?,
            candidate: to_c_str(&self.candidate)?,
        })
    }
}

impl TryFrom<&RawRTCIceCandidate> for RTCIceCandidate {
    type Error = StringError;

    fn try_from(value: &RawRTCIceCandidate) -> Result<Self, Self::Error> {
        Ok(RTCIceCandidate {
            sdp_mline_index: value.sdp_mline_index as u8,
            sdp_mid: from_c_str(value.sdp_mid)?,
            candidate: from_c_str(value.candidate)?,
        })
    }
}

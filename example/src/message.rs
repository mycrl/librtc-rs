use serde::*;
use batrachia::*;
use std::convert::*;
use anyhow::{
    Result,
    anyhow,
};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub kind: String,
    sdp: Option<String>,
    sdpMLineIndex: Option<u8>,
    candidate: Option<String>,
    sdpMid: Option<String>,
}

impl Payload {
    pub fn from_str(str: &str) -> Result<Self> {
        Ok(serde_json::from_str(str)?)
    }

    pub fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TryInto<RTCSessionDescription> for Payload {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RTCSessionDescription, Self::Error> {
        Ok(RTCSessionDescription {
            kind: RTCSessionDescriptionType::Offer,
            sdp: self.sdp.ok_or(anyhow!(""))?.clone(),
        })
    }
}

impl TryInto<RTCIceCandidate> for Payload {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RTCIceCandidate, Self::Error> {
        Ok(RTCIceCandidate {
            sdp_mid: self.sdpMid.ok_or(anyhow!(""))?.clone(),
            candidate: self.candidate.ok_or(anyhow!(""))?.clone(),
            sdp_mline_index: self.sdpMLineIndex.ok_or(anyhow!(""))?,
        })
    }
}

impl From<RTCSessionDescription> for Payload {
    fn from(value: RTCSessionDescription) -> Self {
        Self {
            sdpMid: None,
            candidate: None,
            sdpMLineIndex: None,
            sdp: Some(value.sdp.clone()),
            kind: match value.kind {
                RTCSessionDescriptionType::Answer => "answer".to_string(),
                RTCSessionDescriptionType::Offer => "offer".to_string(),
                RTCSessionDescriptionType::PrAnswer => "prAnswer".to_string(),
                RTCSessionDescriptionType::Rollback => "rollback".to_string(),
            },
        }
    }
}

impl From<RTCIceCandidate> for Payload {
    fn from(value: RTCIceCandidate) -> Self {
        Self {
            sdp: None,
            kind: "candidate".to_string(),
            sdpMLineIndex: Some(value.sdp_mline_index),
            candidate: Some(value.candidate),
            sdpMid: Some(value.sdp_mid),
        }
    }
}

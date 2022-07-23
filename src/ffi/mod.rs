mod raw;

use anyhow::Result;
use libc::*;
use std::ffi::CString;

#[derive(Default)]
pub struct RTCIceServer {
    credential: Option<CString>,
    username: Option<CString>,
    urls: Option<Vec<*const c_char>>,
    raw_urls: Vec<CString>,
}

impl RTCIceServer {
    pub fn set_credential(&mut self, credential: &str) {
        self.credential = Some(CString::new(credential).unwrap());
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = Some(CString::new(username).unwrap());
    }

    pub fn set_urls(&mut self, urls: &[&str]) {
        self.raw_urls = urls.iter().map(|url| CString::new(*url).unwrap()).collect();
        self.urls = Some(
            self.raw_urls
                .iter()
                .map(|url| url.as_c_str().as_ptr())
                .collect(),
        );
    }

    pub fn as_raw(&self) -> raw::RTCIceServer {
        raw::RTCIceServer {
            credential: self.credential.as_ref().map(|c| c.as_c_str().as_ptr()),
            username: self.username.as_ref().map(|u| u.as_c_str().as_ptr()),
            urls: self.urls.as_ref().map(|u| u.as_ptr()),
            urls_size: match &self.urls {
                Some(urls) => urls.len() as c_int,
                None => 0,
            },
        }
    }
}

pub struct RTCConfiguration {
    pub bundle_policy: Option<raw::BUNDLE_POLICY>,
    pub ice_transport_policy: Option<raw::ICE_TRANSPORT_POLICY>,
    pub peer_identity: Option<CString>,
    pub rtcp_mux_policy: Option<raw::RTCP_MUX_POLICY>,
    pub ice_servers: Option<Vec<RTCIceServer>>,
    pub ice_candidate_pool_size: Option<u8>,
}

pub struct RTCPeerConnection {
    raw: *const raw::RTCPeerConnection,
}

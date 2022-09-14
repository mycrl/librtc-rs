use super::promisify::CreateSessionDescriptionPromisify;
use super::raw;
use super::rtc_peerconnection_configure::*;
use super::rtc_session_description::*;
use anyhow::{anyhow, Result};
use futures::task::AtomicWaker;
use libc::*;
use std::convert::{TryFrom, TryInto};
use std::ffi::CString;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;
use std::task::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ConnectionState {
    New,
    Checking,
    Connected,
    Disconnected,
    Close,
    Failed,
}

pub type RawRTCPeerConnection = c_void;

/// RTCPeerConnection
///
/// The RTCPeerConnection interface represents a WebRTC connection between the
/// local computer and a remote peer. It provides methods to connect to a remote
/// peer, maintain and monitor the connection, and close the connection once
/// it's no longer needed.
pub struct RTCPeerConnection<'a> {
    pub(crate) raw: &'a RawRTCPeerConnection,
}

impl<'a> RTCPeerConnection<'a> {
    pub fn run() {
        raw::safe_rtc_run()
    }

    pub fn new(config: RTCConfiguration) -> Result<Self> {
        raw::safe_create_rtc_peerconnection(config)
    }

    pub fn create_answer(&self) -> CreateSessionDescriptionPromisify {
        raw::safe_rtc_create_answer(self.raw)
    }

    pub fn create_offer(&self) -> CreateSessionDescriptionPromisify {
        raw::safe_rtc_create_offer(self.raw)
    }
}

impl Drop for RTCPeerConnection<'_> {
    fn drop(&mut self) {
        raw::safe_rtc_close(self.raw)
    }
}

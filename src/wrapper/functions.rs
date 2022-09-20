use super::events::*;
use super::promisify::*;
use super::rtc_peerconnection::*;
use super::rtc_peerconnection_configure::*;
use super::rtc_session_description::*;
use anyhow::{anyhow, Result};
use libc::*;

#[link(name = "rtc_wrapper")]
extern "C" {
    pub(crate) fn rtc_run();
    pub(crate) fn rtc_close(peer: *const RawRTCPeerConnection);

    pub(crate) fn create_rtc_peer_connection(
        config: *const RawRTCPeerConnectionConfigure,
        eventer: RawEvents,
    ) -> *const RawRTCPeerConnection;

    pub(crate) fn rtc_create_answer(
        peer: *const RawRTCPeerConnection,
        callback: extern "C" fn(*const c_char, *const RawRTCSessionDescription, *mut c_void),
        ctx: *const c_void,
    );

    pub(crate) fn rtc_create_offer(
        peer: *const RawRTCPeerConnection,
        callback: extern "C" fn(*const c_char, *const RawRTCSessionDescription, *mut c_void),
        ctx: *const c_void,
    );
}

pub fn safe_rtc_run() {
    unsafe { rtc_run() }
}

pub fn safe_rtc_close(peer: &RawRTCPeerConnection) {
    unsafe { rtc_close(peer) }
}

pub fn safe_create_rtc_peerconnection<'a>(
    config: &RTCConfiguration,
    eventer: RawEvents,
) -> Result<&'a RawRTCPeerConnection> {
    let raw_config: RawRTCPeerConnectionConfigure = config.into();
    let raw = unsafe { create_rtc_peer_connection(&raw_config, eventer) };

    if raw.is_null() {
        Err(anyhow!("create peerconnection failed!"))
    } else {
        Ok(unsafe { &*raw })
    }
}

pub fn safe_rtc_create_answer<'a>(
    peer: &'a RawRTCPeerConnection,
) -> CreateSessionDescriptionPromisify<'a> {
    CreateSessionDescriptionPromisify::new(peer, SessionDescriptionKind::Answer)
}

pub fn safe_rtc_create_offer<'a>(
    peer: &'a RawRTCPeerConnection,
) -> CreateSessionDescriptionPromisify<'a> {
    CreateSessionDescriptionPromisify::new(peer, SessionDescriptionKind::Offer)
}

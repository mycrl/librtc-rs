use super::events::*;
use super::sys;
use super::promisify::*;
use super::rtc_peerconnection_configure::*;
use super::rtc_session_description::RTCSessionDescription;
use anyhow::Result;
use libc::*;
use std::sync::Arc;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ConnectionState {
    New,
    Connecting,
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
    pub eventer: Arc<Eventer>,
}

impl<'a> RTCPeerConnection<'a> {
    /// By default, RTCPeerConnection::run() calls Thread::Current()->Run().
    /// To receive and dispatch messages, call ProcessMessages occasionally.
    pub fn run() {
        sys::safe_rtc_run()
    }

    /// The RTCPeerConnection constructor returns a newly-created
    /// RTCPeerConnection, which represents a connection between the local
    /// device and a remote peer.
    pub fn new(config: &RTCConfiguration) -> Result<Self> {
        let eventer = Arc::new(Eventer::new());
        let raw = sys::safe_create_rtc_peerconnection(config, eventer.ctx.get_raw())?;
        Ok(Self { raw, eventer })
    }

    /// The create_offer() method of the RTCPeerConnection interface initiates
    /// the creation of an SDP offer for the purpose of starting a new WebRTC
    /// connection to a remote peer. The SDP offer includes information about
    /// any MediaStreamTrack objects already attached to the WebRTC session,
    /// codec, and options supported by the browser, and any candidates already
    /// gathered by the ICE agent, for the purpose of being sent over the
    /// signaling channel to a potential peer to request a connection or to
    /// update the configuration of an existing connection.
    pub fn create_offer(&self) -> CreateDescriptionFuture {
        CreateDescriptionFuture::new(self.raw, CreateDescriptionKind::Offer)
    }

    /// The create_answer() method on the RTCPeerConnection interface creates an
    /// SDP answer to an offer received from a remote peer during the
    /// offer/answer negotiation of a WebRTC connection. The answer contains
    /// information about any media already attached to the session, codecs and
    /// options supported by the browser, and any ICE candidates already gathered.
    /// The answer is delivered to the returned Future, and should then be sent
    /// to the source of the offer to continue the negotiation process.
    pub fn create_answer(&self) -> CreateDescriptionFuture {
        CreateDescriptionFuture::new(self.raw, CreateDescriptionKind::Answer)
    }

    pub fn set_local_description<'b>(
        &'b self,
        desc: &'b RTCSessionDescription,
    ) -> SetDescriptionFuture<'b> {
        SetDescriptionFuture::new(self.raw, desc, SetDescriptionKind::Local)
    }

    pub fn set_remote_description<'b>(
        &'b self,
        desc: &'b RTCSessionDescription,
    ) -> SetDescriptionFuture<'b> {
        SetDescriptionFuture::new(self.raw, desc, SetDescriptionKind::Remote)
    }
}

impl Drop for RTCPeerConnection<'_> {
    fn drop(&mut self) {
        sys::safe_rtc_close(self.raw)
    }
}

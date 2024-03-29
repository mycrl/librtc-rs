use std::fmt::Debug;

use crate::{
    media_stream_track::RawMediaStreamTrack, rtc_datachannel::RawRTCDataChannel,
    rtc_icecandidate::RawRTCIceCandidate, DataChannel, MediaStreamTrack, RTCDataChannel,
    RTCIceCandidate,
};

/// This state essentially represents the aggregate state of all ICE
/// transports (which are of type RTCIceTransport or RTCDtlsTransport)
/// being used by the connection.
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum PeerConnectionState {
    /// At least one of the connection's ICE transports
    /// (RTCIceTransport or RTCDtlsTransport objects) is in the new state,
    /// and none of them are in one of the following states: connecting,
    /// checking, failed, disconnected, or all of the connection's
    /// transports are in the closed state.
    New,
    /// One or more of the ICE transports are currently in the process of
    /// establishing a connection; that is, their iceConnectionState is
    /// either checking or connected, and no transports are in the failed
    /// state.
    Connecting,
    /// Every ICE transport used by the connection is either in use
    /// (state connected or completed) or is closed (state closed); in
    /// addition, at least one transport is either connected or completed.
    Connected,
    /// At least one of the ICE transports for the connection is in the
    /// disconnected state and none of the other transports are in the state
    /// failed, connecting, or checking.
    Disconnected,
    /// One or more of the ICE transports on the connection is in the
    /// failed state.
    Failed,
    /// The RTCPeerConnection is closed.
    Close,
}

/// Describes the state of the signaling process at the local end
/// of the connection when connecting or reconnecting to another peer.
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum SignalingState {
    /// There is no ongoing exchange of offer and answer underway.
    Stable,
    /// The local peer has called RTCPeerConnection.setLocalDescription(),
    /// passing in SDP representing an offer (usually created by calling
    /// RTCPeerConnection.createOffer()), and the offer has been applied
    /// successfully.
    HaveLocalOffer,
    /// The offer sent by the remote peer has been applied and an answer
    /// has been created (usually by calling RTCPeerConnection.createAnswer())
    /// and applied by calling RTCPeerConnection.setLocalDescription().
    /// This provisional answer describes the supported media formats and
    /// so forth, but may not have a complete set of ICE candidates included.
    /// Further candidates will be delivered separately later.
    HaveLocalPrAnswer,
    /// The remote peer has created an offer and used the signaling server
    /// to deliver it to the local peer, which has set the offer as the remote
    /// description by calling RTCPeerConnection.setRemoteDescription().
    HaveRemoteOffer,
    /// A provisional answer has been received and successfully applied
    /// in response to an offer previously sent and established by calling
    /// setLocalDescription().
    HaveRemotePrAnswer,
    /// The RTCPerrConnection has been closed.
    Closed,
}

/// Describes the ICE collection status of the connection.
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum IceGatheringState {
    /// The peer connection was just created and hasn't done any
    /// networking yet.
    New,
    /// The ICE agent is in the process of gathering candidates
    /// for the connection.
    Gathering,
    /// The ICE agent has finished gathering candidates. If
    /// something happens that requires collecting new candidates,
    /// such as a new interface being added or the addition of a
    /// new ICE server, the state will revert to gathering to
    /// gather those candidates.
    Complete,
}

/// It describes the current state of the ICE agent and its connection to
/// the ICE server.
#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum IceConnectionState {
    /// The ICE agent is gathering addresses or is waiting to be given
    /// remote candidates through calls to RTCPeerConnection.addIceCandidate().
    New,
    /// The ICE agent has been given one or more remote candidates and is
    /// checking pairs of local and remote candidates against one another
    /// to try to find a compatible match, but has not yet found a pair
    /// which will allow the peer connection to be made. It is possible
    /// that gathering of candidates is also still underway.
    Checking,
    /// A usable pairing of local and remote candidates has been found for
    /// all components of the connection, and the connection has been
    /// established. It is possible that gathering is still underway, and
    /// it is also possible that the ICE agent is still checking candidates
    /// against one another looking for a better connection to use.
    Connected,
    /// The ICE agent has finished gathering candidates, has checked all
    /// pairs against one another, and has found a connection for all
    /// components.
    Completed,
    /// The ICE candidate has checked all candidates pairs against one
    /// another and has failed to find compatible matches for all components
    /// of the connection. It is, however, possible that the ICE agent did
    /// find compatible connections for some components.
    Failed,
    /// Checks to ensure that components are still connected failed for at
    /// least one component of the RTCPeerConnection. This is a less stringent
    /// test than failed and may trigger intermittently and resolve just as
    /// spontaneously on less reliable networks, or during temporary
    /// disconnections. When the problem resolves, the connection may return
    /// to the connected state.
    Disconnected,
    /// The ICE agent for this RTCPeerConnection has shut down and is no
    /// longer handling requests.
    Closed,
    Max,
}

/// PeerConnection callback interface, used for RTCPeerConnection events.
/// Application should implement these methods.
#[allow(unused)]
pub trait Observer {
    /// A signalingstatechange event is sent to an RTCPeerConnection to notify
    /// it that its signaling state, as indicated by the signalingState
    /// property, has changed.
    fn on_signaling_change(&self, state: SignalingState) {}
    /// The connectionstatechange event is sent to the onconnectionstatechange
    /// event handler on an RTCPeerConnection object after a new track has been
    /// added to an RTCRtpReceiver which is part of the connection.
    fn on_connection_change(&self, state: PeerConnectionState) {}
    /// The icegatheringstatechange event is sent to the
    /// onicegatheringstatechange event handler on an RTCPeerConnection when
    /// the state of the ICE candidate gathering process changes. This
    /// signifies that the value of the connection's iceGatheringState property
    /// has changed.
    ///
    /// When ICE first starts to gather connection candidates, the value
    /// changes from new to gathering to indicate that the process of
    /// collecting candidate configurations for the connection has begun. When
    /// the value changes to complete, all of the transports that make up the
    /// RTCPeerConnection have finished gathering ICE candidates.
    fn on_ice_gathering_change(&self, state: IceGatheringState) {}
    /// An icecandidate event is sent to an RTCPeerConnection when an
    /// RTCIceCandidate has been identified and added to the local peer by a
    /// call to RTCPeerConnection.setLocalDescription(). The event handler
    /// should transmit the candidate to the remote peer over the signaling
    /// channel so the remote peer can add it to its set of remote candidates.
    fn on_ice_candidate(&self, candidate: RTCIceCandidate) {}
    /// A negotiationneeded event is sent to the RTCPeerConnection when
    /// negotiation of the connection through the signaling channel is
    /// required. This occurs both during the initial setup of the connection
    /// as well as any time a change to the communication environment requires
    /// reconfiguring the connection.
    fn on_renegotiation_needed(&self) {}
    /// An iceconnectionstatechange event is sent to an RTCPeerConnection
    /// object each time the ICE connection state changes during the
    /// negotiation process. The new ICE connection state is available in the
    /// object's iceConnectionState property.
    fn on_ice_connection_change(&self, state: IceConnectionState) {}
    /// The track event is sent to the ontrack event handler on
    /// RTCPeerConnections after a new track has been added to an
    /// RTCRtpReceiver which is part of the connection.
    fn on_track(&self, track: MediaStreamTrack) {}
    /// A datachannel event is sent to an RTCPeerConnection instance when an
    /// RTCDataChannel has been added to the connection, as a result of the
    /// remote peer calling RTCPeerConnection.createDataChannel().
    fn on_data_channel(&self, channel: RTCDataChannel) {}
}

/// wrapper observer trait impl.
pub struct ObserverRef {
    data: Box<dyn Observer>,
}

impl ObserverRef {
    pub fn new<T: Observer + 'static>(data: T) -> Self {
        Self {
            data: Box::new(data),
        }
    }
}

/// rtc peer connection observer events callback ref.
#[repr(C)]
#[rustfmt::skip]
pub(crate) struct TEvents {
    on_signaling_change: extern "C" fn(*mut ObserverRef, SignalingState),
    on_datachannel: extern "C" fn(*mut ObserverRef, *const RawRTCDataChannel),
    on_ice_gathering_change: extern "C" fn(*mut ObserverRef, IceGatheringState),
    on_ice_candidate: extern "C" fn(*mut ObserverRef, *const RawRTCIceCandidate),
    on_renegotiation_needed: extern "C" fn(*mut ObserverRef),
    on_ice_connection_change: extern "C" fn(*mut ObserverRef, IceConnectionState),
    on_track: extern "C" fn(*mut ObserverRef, *const RawMediaStreamTrack),
    on_connection_change: extern "C" fn(*mut ObserverRef, PeerConnectionState),
}

/// events callback const ref.
pub(crate) const EVENTS: TEvents = TEvents {
    on_signaling_change,
    on_datachannel,
    on_ice_gathering_change,
    on_ice_candidate,
    on_renegotiation_needed,
    on_ice_connection_change,
    on_track,
    on_connection_change,
};

extern "C" fn on_signaling_change(ctx: *mut ObserverRef, state: SignalingState) {
    assert!(!ctx.is_null());
    (unsafe { &mut *ctx }).data.on_signaling_change(state);
}

extern "C" fn on_connection_change(ctx: *mut ObserverRef, state: PeerConnectionState) {
    assert!(!ctx.is_null());
    (unsafe { &mut *ctx }).data.on_connection_change(state);
}

extern "C" fn on_ice_gathering_change(ctx: *mut ObserverRef, state: IceGatheringState) {
    assert!(!ctx.is_null());
    (unsafe { &mut *ctx }).data.on_ice_gathering_change(state);
}

extern "C" fn on_ice_candidate(ctx: *mut ObserverRef, candidate: *const RawRTCIceCandidate) {
    assert!(!ctx.is_null());
    assert!(!candidate.is_null());
    let candidate = RTCIceCandidate::try_from(unsafe { &*candidate }).unwrap();
    (unsafe { &mut *ctx }).data.on_ice_candidate(candidate);
}

extern "C" fn on_renegotiation_needed(ctx: *mut ObserverRef) {
    assert!(!ctx.is_null());
    (unsafe { &mut *ctx }).data.on_renegotiation_needed();
}

extern "C" fn on_ice_connection_change(ctx: *mut ObserverRef, state: IceConnectionState) {
    assert!(!ctx.is_null());
    (unsafe { &mut *ctx }).data.on_ice_connection_change(state);
}

extern "C" fn on_datachannel(ctx: *mut ObserverRef, channel: *const RawRTCDataChannel) {
    assert!(!ctx.is_null() && !channel.is_null());
    let channel = DataChannel::from_raw(channel);
    (unsafe { &mut *ctx }).data.on_data_channel(channel);
}

extern "C" fn on_track(ctx: *mut ObserverRef, track: *const RawMediaStreamTrack) {
    assert!(!ctx.is_null() && !track.is_null());
    let track = MediaStreamTrack::from_raw(track);
    (unsafe { &mut *ctx }).data.on_track(track);
}

mod create_description;
mod set_description;

pub use create_description::*;
pub use set_description::*;

use anyhow::Result;
use futures::task::AtomicWaker;
use std::{
    cell::UnsafeCell,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::*, fmt::Debug,
};

use super::{
    media_stream_track::*,
    rtc_datachannel::*,
    rtc_icecandidate::*,
};

use tokio::sync::{
    broadcast::*,
};

pub struct ObserverPromisify<T>
where
    T: ObserverPromisifyExt,
{
    waker: Arc<AtomicWaker>,
    begin: bool,
    ext: T,
}

impl<T> Future for ObserverPromisify<T>
where
    T: ObserverPromisifyExt + Unpin,
{
    type Output = anyhow::Result<T::Output>;

    #[rustfmt::skip]
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output> {
        let mut this = self.as_mut();
        this.waker.register(cx.waker());
        if !this.begin {
            match this.ext.handle(this.waker.clone()) {
                Err(e) => Poll::Ready(Err(e)),
                Ok(_) => {
                    this.begin = true;
                    Poll::Pending
                },
            }
        } else {
            this.ext
                .wake()
                .map(Poll::Ready)
                .unwrap_or(Poll::Pending)
        }
    }
}

pub struct Events<T> {
    receiver: Receiver<T>,
    sender: Sender<T>,
}

impl<T: Clone + Debug> Clone for Events<T> {
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.resubscribe(),
            sender: self.sender.clone(),
        }
    }
}

impl<T: Clone + Debug> Events<T> {
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = channel(capacity);
        Self {
            receiver,
            sender,
        }
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await.ok()
    }

    pub fn send(&self, value: T) {
        self.sender.send(value).unwrap();
    }
}

#[repr(C)]
#[rustfmt::skip]
pub(crate) struct IObserver {
    ctx: *const ObserverContext,

    on_signaling_change: extern "C" fn(*const ObserverContext, SignalingState),
    on_datachannel: extern "C" fn(*const ObserverContext, *const RawRTCDataChannel),
    on_ice_gathering_change: extern "C" fn(*const ObserverContext, IceGatheringState),
    on_ice_candidate: extern "C" fn(*const ObserverContext, *const RawRTCIceCandidate),
    on_renegotiation_needed: extern "C" fn(*const ObserverContext),
    on_ice_connection_change: extern "C" fn(*const ObserverContext, IceConnectionState),
    on_track: extern "C" fn(*const ObserverContext, *const RawMediaStreamTrack),
    on_connection_change: extern "C" fn(*const ObserverContext, PeerConnectionState),
}

impl IObserver {
    pub fn new(ctx: *const ObserverContext) -> Self {
        Self {
            ctx,
            on_signaling_change,
            on_connection_change,
            on_ice_gathering_change,
            on_ice_candidate,
            on_renegotiation_needed,
            on_ice_connection_change,
            on_datachannel,
            on_track,
        }
    }
}

pub trait ObserverPromisifyExt {
    type Output;
    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<()>;
    fn wake(&self) -> Option<Result<Self::Output>>;
}

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

pub struct ObserverContext {
    signaling_change: Events<SignalingState>,
    connection_change: Events<PeerConnectionState>,
    ice_gathering_change: Events<IceGatheringState>,
    ice_candidate: Events<RTCIceCandidate>,
    renegotiation_needed: Events<()>,
    ice_connection_change: Events<IceConnectionState>,
    track: Events<Arc<MediaStreamTrack>>,
    data_channel: Events<Arc<RTCDataChannel>>,
}

impl ObserverContext {
    fn new() -> Self {
        Self {
            connection_change: Events::new(10),
            signaling_change: Events::new(10),
            ice_gathering_change: Events::new(10),
            ice_candidate: Events::new(10),
            renegotiation_needed: Events::new(10),
            ice_connection_change: Events::new(10),
            data_channel: Events::new(10),
            track: Events::new(10),
        }
    }
}

/// RTCPeerConnection callback interface.
///
/// used for RTCPeerConnection events.
pub struct Observer {
    /// A signalingstatechange event is sent to an RTCPeerConnection to notify
    /// it that its signaling state, as indicated by the signalingState
    /// property, has changed.
    pub signaling_change: Events<SignalingState>,
    /// The connectionstatechange event is sent to the onconnectionstatechange
    /// event handler on an RTCPeerConnection object after a new track has been
    /// added to an RTCRtpReceiver which is part of the connection.
    pub connection_change: Events<PeerConnectionState>,
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
    pub ice_gathering_change: Events<IceGatheringState>,
    /// An icecandidate event is sent to an RTCPeerConnection when an
    /// RTCIceCandidate has been identified and added to the local peer by a
    /// call to RTCPeerConnection.setLocalDescription(). The event handler
    /// should transmit the candidate to the remote peer over the signaling
    /// channel so the remote peer can add it to its set of remote candidates.
    pub ice_candidate: Events<RTCIceCandidate>,
    /// A negotiationneeded event is sent to the RTCPeerConnection when
    /// negotiation of the connection through the signaling channel is
    /// required. This occurs both during the initial setup of the connection
    /// as well as any time a change to the communication environment requires
    /// reconfiguring the connection.
    pub renegotiation_needed: Events<()>,
    /// An iceconnectionstatechange event is sent to an RTCPeerConnection
    /// object each time the ICE connection state changes during the
    /// negotiation process. The new ICE connection state is available in the
    /// object's iceConnectionState property.
    pub ice_connection_change: Events<IceConnectionState>,
    /// The track event is sent to the ontrack event handler on
    /// RTCPeerConnections after a new track has been added to an
    /// RTCRtpReceiver which is part of the connection.
    pub track: Events<Arc<MediaStreamTrack>>,
    /// A datachannel event is sent to an RTCPeerConnection instance when an
    /// RTCDataChannel has been added to the connection, as a result of the
    /// remote peer calling RTCPeerConnection.createDataChannel().
    pub data_channel: Events<Arc<RTCDataChannel>>,

    raw_ptr: UnsafeCell<Option<*const IObserver>>,
    ctx: *const ObserverContext,
}

unsafe impl Send for Observer {}
unsafe impl Sync for Observer {}

impl Observer {
    /// Create a peer connection event listener.
    pub fn new() -> Self {
        let ctx = ObserverContext::new();
        Self {
            track: ctx.track.clone(),
            connection_change: ctx.connection_change.clone(),
            signaling_change: ctx.signaling_change.clone(),
            ice_gathering_change: ctx.ice_gathering_change.clone(),
            ice_candidate: ctx.ice_candidate.clone(),
            renegotiation_needed: ctx.renegotiation_needed.clone(),
            ice_connection_change: ctx.ice_connection_change.clone(),
            data_channel: ctx.data_channel.clone(),
            raw_ptr: UnsafeCell::new(None),
            ctx: Box::into_raw(Box::new(ctx)),
        }
    }

    pub(crate) fn get_raw(&self) -> *const IObserver {
        let raw_ptr = unsafe { &mut *self.raw_ptr.get() };
        if let Some(ptr) = raw_ptr {
            return *ptr;
        }

        let raw = Box::into_raw(Box::new(IObserver::new(self.ctx)));
        let _ = raw_ptr.insert(raw);
        raw
    }
}

impl Default for Observer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Observer {
    fn drop(&mut self) {
        let raw_ptr = unsafe { *self.raw_ptr.get() };
        if let Some(ptr) = raw_ptr {
            let _ = unsafe {
                Box::from_raw(ptr as *mut IObserver)
            };
        }

        let _ = unsafe {
            Box::from_raw(self.ctx as *mut ObserverContext)
        };
    }
}

extern "C" fn on_signaling_change(
    ctx: *const ObserverContext,
    state: SignalingState,
) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        ctx.signaling_change.send(state);
    }
}

extern "C" fn on_connection_change(
    ctx: *const ObserverContext,
    state: PeerConnectionState,
) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        ctx.connection_change.send(state);
    }
}

extern "C" fn on_ice_gathering_change(
    ctx: *const ObserverContext,
    state: IceGatheringState,
) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        ctx.ice_gathering_change.send(state);
    }
}

extern "C" fn on_ice_candidate(
    ctx: *const ObserverContext,
    candidate: *const RawRTCIceCandidate,
) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        if let Some(candidate) = unsafe { candidate.as_ref() } {
            if let Ok(candidate) = RTCIceCandidate::try_from(candidate)
            {
                ctx.ice_candidate.send(candidate);
            }
        }
    }
}

extern "C" fn on_renegotiation_needed(ctx: *const ObserverContext) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        ctx.renegotiation_needed.send(());
    }
}

extern "C" fn on_ice_connection_change(
    ctx: *const ObserverContext,
    state: IceConnectionState,
) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        ctx.ice_connection_change.send(state);
    }
}

extern "C" fn on_datachannel(
    ctx: *const ObserverContext,
    channel: *const RawRTCDataChannel,
) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        if let Some(channel) = unsafe { channel.as_ref() } {
            ctx.data_channel
                .send(RTCDataChannel::from_raw(channel));
        }
    }
}

extern "C" fn on_track(
    ctx: *const ObserverContext,
    track: *const RawMediaStreamTrack,
) {
    if let Some(ctx) = unsafe { ctx.as_ref() } {
        if let Some(track) = unsafe { track.as_ref() } {
            ctx.track
                .send(MediaStreamTrack::from_raw(track));
        }
    }
}

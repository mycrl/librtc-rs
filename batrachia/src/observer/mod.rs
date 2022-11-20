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
    task::*,
};

use super::{
    media_stream_track::*,
    rtc_icecandidate::*,
};

use tokio::sync::{
    broadcast::*,
    Mutex,
};

pub trait ObserverPromisifyExt {
    type Output;
    fn handle(&self, waker: Arc<AtomicWaker>) -> Result<()>;
    fn wake(&self) -> Option<Result<Self::Output>>;
}

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
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut();
        this.waker.register(cx.waker());
        if !this.begin {
            match this.ext.handle(this.waker.clone()) {
                Err(e) => Poll::Ready(Err(e)),
                Ok(_) => {
                    this.begin = true;
                    Poll::Pending
                }
            }
        } else {
            this.ext
                .wake()
                .map(Poll::Ready)
                .unwrap_or(Poll::Pending)
        }
    }
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
    /// either checking or connected, and no transports are in the failed state.
    Connecting,
    /// Every ICE transport used by the connection is either in use 
    /// (state connected or completed) or is closed (state closed); in addition, 
    /// at least one transport is either connected or completed.
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
    connection_change_sdr: Sender<PeerConnectionState>,
    signaling_change_sdr: Sender<SignalingState>,
    ice_gathering_change_sdr: Sender<IceGatheringState>,
    ice_candidate_sdr: Sender<RTCIceCandidate>,
    renegotiation_needed_sdr: Sender<()>,
    ice_connection_change_sdr: Sender<IceConnectionState>,
    track_sdr: Sender<Arc<MediaStreamTrack>>,
}

#[repr(C)]
pub(crate) struct IObserver {
    ctx: *mut ObserverContext,

    on_signaling_change: extern "C" fn(*mut ObserverContext, SignalingState),
    on_datachannel: extern "C" fn(*mut ObserverContext),
    on_ice_gathering_change: extern "C" fn(*mut ObserverContext, IceGatheringState),
    on_ice_candidate: extern "C" fn(*mut ObserverContext, *const RawRTCIceCandidate),
    on_renegotiation_needed: extern "C" fn(*mut ObserverContext),
    on_ice_connection_change: extern "C" fn(*mut ObserverContext, IceConnectionState),
    on_track: extern "C" fn(*mut ObserverContext, *const RawMediaStreamTrack),
    on_connection_change: extern "C" fn(*mut ObserverContext, PeerConnectionState),
}

#[derive(Clone)]
pub struct ChennelRecv<T> {
    inner: Arc<Mutex<Receiver<T>>>,
}

impl<T: Clone> ChennelRecv<T> {
    pub fn new(capacity: usize) -> (Sender<T>, Self) {
        let (sender, inner) = channel(capacity);
        (
            sender,
            Self {
                inner: Arc::new(Mutex::new(inner)),
            },
        )
    }

    pub async fn recv(&self) -> Option<T> {
        self.inner.lock().await.recv().await.ok()
    }
}

/// RTCPeerConnection callback interface.
///
/// used for RTCPeerConnection events.
pub struct Observer {
    pub signaling_change: ChennelRecv<SignalingState>,
    pub connection_change: ChennelRecv<PeerConnectionState>,
    pub ice_gathering_change: ChennelRecv<IceGatheringState>,
    pub ice_candidate: ChennelRecv<RTCIceCandidate>,
    pub renegotiation_needed: ChennelRecv<()>,
    pub ice_connection_change: ChennelRecv<IceConnectionState>,
    pub track: ChennelRecv<Arc<MediaStreamTrack>>,

    ctx: ObserverContext,

    // box mannager
    raw_ptr: UnsafeCell<Option<*const IObserver>>,
}

unsafe impl Send for Observer {}
unsafe impl Sync for Observer {}

impl Observer {
    pub fn new() -> Self {
        let (connection_change_sdr, connection_change) = ChennelRecv::new(1);
        let (signaling_change_sdr, signaling_change) = ChennelRecv::new(1);
        let (ice_gathering_change_sdr, ice_gathering_change) = ChennelRecv::new(1);
        let (ice_candidate_sdr, ice_candidate) = ChennelRecv::new(1);
        let (renegotiation_needed_sdr, renegotiation_needed) = ChennelRecv::new(1);
        let (ice_connection_change_sdr, ice_connection_change) = ChennelRecv::new(1);
        let (track_sdr, track) = ChennelRecv::new(1);

        let ctx = ObserverContext {
            connection_change_sdr,
            signaling_change_sdr,
            ice_gathering_change_sdr,
            ice_candidate_sdr,
            renegotiation_needed_sdr,
            ice_connection_change_sdr,
            track_sdr,
        };

        Self {
            ice_gathering_change,
            connection_change,
            signaling_change,
            ice_candidate,
            renegotiation_needed,
            ice_connection_change,
            track,

            raw_ptr: UnsafeCell::new(None),
            ctx,
        }
    }

    pub(crate) fn get_raw(&self) -> *const IObserver {
        let raw_ptr = unsafe { &mut *self.raw_ptr.get() };
        if let Some(ptr) = raw_ptr {
            return *ptr;
        }

        let raw = Box::into_raw(Box::new(IObserver {
            ctx: &self.ctx as *const ObserverContext as *mut ObserverContext,
            on_signaling_change,
            on_connection_change,
            on_ice_gathering_change,
            on_ice_candidate,
            on_renegotiation_needed,
            on_ice_connection_change,
            on_datachannel,
            on_track,
        }));

        let  _ = raw_ptr.insert(raw);
        raw
    }
}

impl Default for Observer {
    fn default() -> Self {
        Self::new()
    }
}

extern "C" fn on_signaling_change(ctx: *mut ObserverContext, state: SignalingState) {
    unsafe { &*ctx }.signaling_change_sdr.send(state).unwrap();
}

extern "C" fn on_connection_change(ctx: *mut ObserverContext, state: PeerConnectionState) {
    unsafe { &*ctx }.connection_change_sdr.send(state).unwrap();
}

extern "C" fn on_ice_gathering_change(ctx: *mut ObserverContext, state: IceGatheringState) {
    unsafe { &*ctx }
        .ice_gathering_change_sdr
        .send(state)
        .unwrap();
}

extern "C" fn on_ice_candidate(ctx: *mut ObserverContext, candidate: *const RawRTCIceCandidate) {
    if !candidate.is_null() {
        unsafe { &*ctx }
            .ice_candidate_sdr
            .send(RTCIceCandidate::try_from(unsafe { &*candidate }).unwrap())
            .unwrap();
    }
}

extern "C" fn on_renegotiation_needed(ctx: *mut ObserverContext) {
    unsafe { &*ctx }.renegotiation_needed_sdr.send(()).unwrap();
}

extern "C" fn on_ice_connection_change(ctx: *mut ObserverContext, state: IceConnectionState) {
    unsafe { &*ctx }
        .ice_connection_change_sdr
        .send(state)
        .unwrap();
}

extern "C" fn on_datachannel(_ctx: *mut ObserverContext) {}

extern "C" fn on_track(ctx: *mut ObserverContext, track: *const RawMediaStreamTrack) {
    if let Ok(t) = MediaStreamTrack::from_raw(track) {
        unsafe { &*ctx }.track_sdr.send(t).unwrap();
    }
}

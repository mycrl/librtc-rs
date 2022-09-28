use super::rtc_datachannel::*;
use std::sync::Arc;
use tokio::sync::broadcast::*;
use tokio::sync::Mutex;

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
// See https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnectionstate
pub enum ConnectionState {
    New,
    Connecting,
    Connected,
    Disconnected,
    Close,
    Failed,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum SignalingState {
    Stable,
    HaveLocalOffer,
    HaveLocalPrAnswer,
    HaveRemoteOffer,
    HaveRemotePrAnswer,
    Closed,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
// See https://w3c.github.io/webrtc-pc/#dom-rtcicegatheringstate
pub enum IceGatheringState {
    IceGatheringNew,
    IceGatheringGathering,
    IceGatheringComplete,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
// See https://w3c.github.io/webrtc-pc/#dom-rtciceconnectionstate
pub enum IceConnectionState {
    IceConnectionNew,
    IceConnectionChecking,
    IceConnectionConnected,
    IceConnectionCompleted,
    IceConnectionFailed,
    IceConnectionDisconnected,
    IceConnectionClosed,
    IceConnectionMax,
}

#[repr(C)]
pub struct RawEvents {
    ctx: *mut EventerContext,
    on_connectionstatechange: extern "C" fn(*mut EventerContext, ConnectionState),
    on_datachannel: extern "C" fn(*mut EventerContext, RawRTCDataChannel),
    on_signalingchange: extern "C" fn(*mut EventerContext, SignalingState),
}

extern "C" fn on_signalingchange(ctx: *mut EventerContext, state: SignalingState) {
    unsafe { &*ctx }.signalingchange_sdr.send(state).unwrap();
}

extern "C" fn on_connectionstatechange(ctx: *mut EventerContext, state: ConnectionState) {
    unsafe { &*ctx }
        .connectionstatechange_sdr
        .send(state)
        .unwrap();
}

extern "C" fn on_datachannel(ctx: *mut EventerContext, datachanel: RawRTCDataChannel) {
    // unsafe { &*ctx }
    //     .datachannel_sdr
    //     .send(Some(datachanel))
    //     .unwrap();
}

pub struct EventerContext {
    connectionstatechange_sdr: Sender<ConnectionState>,
    datachannel_sdr: Sender<RTCDataChannel>,
    signalingchange_sdr: Sender<SignalingState>,
}

impl EventerContext {
    pub fn get_raw(&self) -> RawEvents {
        RawEvents {
            ctx: self as *const Self as *mut Self,
            on_connectionstatechange: on_connectionstatechange,
            on_datachannel: on_datachannel,
            on_signalingchange: on_signalingchange,
        }
    }
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

pub struct Eventer {
    pub signalingchange_rev: ChennelRecv<SignalingState>,
    pub connectionstatechange_rev: ChennelRecv<ConnectionState>,
    pub datachannel_rev: ChennelRecv<RTCDataChannel>,
    pub(crate) ctx: EventerContext,
}

impl Eventer {
    pub fn new() -> Self {
        let (connectionstatechange_sdr, connectionstatechange_rev) = ChennelRecv::new(1);
        let (datachannel_sdr, datachannel_rev) = ChennelRecv::new(1);
        let (signalingchange_sdr, signalingchange_rev) = ChennelRecv::new(1);
        Self {
            connectionstatechange_rev,
            datachannel_rev,
            signalingchange_rev,
            ctx: EventerContext {
                connectionstatechange_sdr,
                datachannel_sdr,
                signalingchange_sdr,
            },
        }
    }
}

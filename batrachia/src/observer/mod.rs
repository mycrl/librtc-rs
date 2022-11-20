mod create_description;
mod set_description;

pub use create_description::*;
pub use set_description::*;

use anyhow::Result;
use futures::task::AtomicWaker;
use std::{
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
                .and_then(|r| Some(Poll::Ready(r)))
                .or_else(|| Some(Poll::Pending))
                .unwrap()
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum PeerConnectionState {
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
pub enum IceGatheringState {
    New,
    Gathering,
    Complete,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum IceConnectionState {
    New,
    Checking,
    Connected,
    Completed,
    Failed,
    Disconnected,
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
pub struct IObserver {
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
    raw_ptr: Option<*const IObserver>,
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

            raw_ptr: None,
            ctx,
        }
    }

    pub fn get_raw(&self) -> *const IObserver {
        if let Some(ptr) = self.raw_ptr {
            return ptr;
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

        unsafe {
            (*(self as *const Self as *mut Self)).raw_ptr = Some(raw);
        }

        raw
    }
}

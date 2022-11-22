mod base;
mod media_stream;
mod media_stream_track;
mod observer;
mod rtc_datachannel;
mod rtc_icecandidate;
mod rtc_peerconnection;
mod rtc_peerconnection_configure;
mod rtc_session_description;
mod video_frame;

pub use media_stream::MediaStream;
pub use media_stream_track::{
    MediaStreamTrack,
    MediaStreamTrackKind,
    MediaStreamTrackSink,
};

pub use rtc_icecandidate::RTCIceCandidate;
pub use rtc_peerconnection::RTCPeerConnection;

pub use video_frame::{
    I420Frame,
    I420Layout,
};

pub use rtc_datachannel::{
    DataChannelOptions,
    DataChannelPriority,
    DataChannelState,
    RTCDataChannel,
    RTCDataChannelSink,
};

pub use rtc_peerconnection_configure::{
    BundlePolicy,
    IceTransportPolicy,
    RTCConfiguration,
    RTCIceServer,
    RtcpMuxPolicy,
};

pub use rtc_session_description::{
    RTCSessionDescription,
    RTCSessionDescriptionType,
};

pub use observer::{
    CreateDescriptionObserver,
    IceConnectionState,
    IceGatheringState,
    Observer,
    ObserverPromisify,
    ObserverPromisifyExt,
    PeerConnectionState,
    SetDescriptionObserver,
    SignalingState,
};

use once_cell::sync::Lazy;
use anyhow::Result;
use tokio::{
    task::JoinHandle,
    runtime::Builder,
};

static THREAD: Lazy<Result<JoinHandle<()>>> = Lazy::new(|| {
    Ok(Builder::new_current_thread()
        .build()?
        .spawn_blocking(RTCPeerConnection::run))
});

pub fn run() -> &'static Lazy<Result<JoinHandle<()>>> {
    &THREAD
}

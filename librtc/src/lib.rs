//! ##### Facilitating high-level interactions between Rust and WebRTC
//！
//! [M99]: https://groups.google.com/g/discuss-webrtc/c/Yf6c3HW4N3k/m/3SC_Hy15BQAJ
//!
//! The rust high-level abstraction binding of Google WebRTC [M99].
//! With WebRTC, you can add real-time communication capabilities to
//! your application that works on top of an open standard. It supports
//! video, voice, and generic data to be sent between peers, allowing
//! developers to build powerful voice- and video-communication solutions.

mod cstr;
mod auto_ptr;
mod audio_frame;
mod video_frame;
mod media_stream;
mod media_stream_track;
mod audio_track;
mod video_track;
mod observer;
mod create_description_observer;
mod set_description_observer;
mod rtc_datachannel;
mod rtc_icecandidate;
mod rtc_peerconnection;
mod rtc_peerconnection_configure;
mod rtc_session_description;
mod promisify;
mod sink;

pub use video_track::VideoTrack;
pub use audio_track::AudioTrack;
pub use media_stream::MediaStream;
pub use media_stream_track::{
    MediaStreamTrack,
    MediaStreamTrackKind,
};

pub use sink::{
    Sinker,
    SinkExt,
};

pub use rtc_icecandidate::RTCIceCandidate;
pub use rtc_peerconnection::RTCPeerConnection;
pub use audio_frame::AudioFrame;
pub use video_frame::VideoFrame;

pub use rtc_datachannel::{
    DataChannelOptions,
    DataChannelPriority,
    DataChannelState,
    RTCDataChannel,
    DataChannel,
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

pub use create_description_observer::CreateDescriptionObserver;
pub use set_description_observer::SetDescriptionObserver;
pub use observer::{
    IceConnectionState,
    IceGatheringState,
    ObserverExt,
    Observer,
    PeerConnectionState,
    SignalingState,
};

pub use promisify::{
    Promisify,
    PromisifyExt,
    SpawnBlocking,
};

extern "C" {
    pub(crate) fn rtc_run();
    pub(crate) fn rtc_exit();
}

/// By default, run() calls Thread::Current()->Run().
/// To receive and dispatch messages.
pub async fn run() {
    SpawnBlocking::new(|| unsafe { rtc_run() }).await
}

pub fn exit() {
    unsafe { rtc_exit() }
}

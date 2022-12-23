//! ##### Facilitating high-level interactions between Rust and WebRTC
//！
//! [M99]: https://groups.google.com/g/discuss-webrtc/c/Yf6c3HW4N3k/m/3SC_Hy15BQAJ
//!
//! The rust high-level abstraction binding of Google WebRTC [M99].
//! With WebRTC, you can add real-time communication capabilities to
//! your application that works on top of an open standard. It supports
//! video, voice, and generic data to be sent between peers, allowing
//! developers to build powerful voice- and video-communication solutions.

mod base;
mod frame;
mod codec;
mod symbols;
mod abstracts;
mod media_stream;
mod media_stream_track;
mod observer;
mod rtc_datachannel;
mod rtc_icecandidate;
mod rtc_peerconnection;
mod rtc_peerconnection_configure;
mod rtc_session_description;
mod stream_ext;

pub use media_stream::MediaStream;
pub use media_stream_track::{
    MediaStreamTrack,
    MediaStreamTrackKind,
    VideoTrack,
    AudioTrack,
};

pub use codec::video_encoder::{
    VideoEncoderFactory,
    VideoEncoder,
};

pub use stream_ext::{
    Sinker,
    SinkExt,
};

pub use rtc_icecandidate::RTCIceCandidate;
pub use rtc_peerconnection::RTCPeerConnection;
pub use frame::{
    AudioFrame,
    VideoFrame,
};

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

pub use observer::{
    CreateDescriptionObserver,
    IceConnectionState,
    IceGatheringState,
    ObserverExt,
    Observer,
    ObserverPromisify,
    ObserverPromisifyExt,
    PeerConnectionState,
    SetDescriptionObserver,
    SignalingState,
};

/// By default, run() calls Thread::Current()->Run().
/// To receive and dispatch messages.
pub fn run() {
    RTCPeerConnection::run()
}

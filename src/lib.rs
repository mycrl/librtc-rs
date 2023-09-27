//! ##### Facilitating high-level interactions between Rust and WebRTC
//！
//! [M99]: https://groups.google.com/g/discuss-webrtc/c/Yf6c3HW4N3k/m/3SC_Hy15BQAJ
//!
//! The rust high-level abstraction binding of Google WebRTC [M99].
//! With WebRTC, you can add real-time communication capabilities to
//! your application that works on top of an open standard. It supports
//! video, voice, and generic data to be sent between peers, allowing
//! developers to build powerful voice- and video-communication solutions.

mod audio_frame;
mod audio_track;
mod auto_ptr;
mod create_description_observer;
mod cstr;
mod media_stream;
mod media_stream_track;
mod observer;
mod promisify;
mod rtc_datachannel;
mod rtc_icecandidate;
mod rtc_peerconnection;
mod rtc_peerconnection_configure;
mod rtc_session_description;
mod set_description_observer;
mod sink;
mod video_frame;
mod video_track;

pub use audio_frame::AudioFrame;
pub use audio_track::AudioTrack;
pub use create_description_observer::{CreateDescriptionObserver, CreateDescriptionError};
pub use media_stream::{MediaStream, MediaStreamError};
pub use media_stream_track::{MediaStreamTrack, MediaStreamTrackKind};
pub use observer::{
    IceConnectionState, IceGatheringState, Observer, PeerConnectionState, SignalingState,
};
pub use promisify::{Promisify, PromisifyExt, SpawnBlocking};
pub use rtc_datachannel::{
    DataChannel, DataChannelOptions, DataChannelPriority, DataChannelState, RTCDataChannel,
};
pub use cstr::StringError;
pub use rtc_icecandidate::RTCIceCandidate;
pub use rtc_peerconnection::{RTCPeerConnection, RTCError};
pub use rtc_peerconnection_configure::{
    BundlePolicy, IceTransportPolicy, RTCConfiguration, RTCIceServer, RtcpMuxPolicy,
};
pub use rtc_session_description::{RTCSessionDescription, RTCSessionDescriptionType};
pub use set_description_observer::{SetDescriptionObserver, SetDescriptionError};
pub use sink::{SinkExt, Sinker};
pub use video_frame::VideoFrame;
pub use video_track::VideoTrack;

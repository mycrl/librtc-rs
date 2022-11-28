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
mod audio_frame;

pub use media_stream::MediaStream;
pub use media_stream_track::{
    MediaStreamTrack,
    MediaStreamTrackKind,
    video_track::VideoTrack,
    video_track::VideoTrackSink,
    audio_track::AudioTrack,
    audio_track::AudioTrackSink,
};

pub use rtc_icecandidate::RTCIceCandidate;
pub use rtc_peerconnection::RTCPeerConnection;
pub use audio_frame::PCMFrames;
pub use video_frame::I420Frame;

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

/// By default, run() calls Thread::Current()->Run().
/// To receive and dispatch messages, call ProcessMessages occasionally.
pub fn run() {
    RTCPeerConnection::run()
}

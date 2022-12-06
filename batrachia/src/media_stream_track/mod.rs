pub(crate) mod audio_track;
pub(crate) mod video_track;

use audio_track::*;
use video_track::*;
use std::sync::Arc;
use libc::*;

extern "C" {
    pub(crate) fn free_media_track(track: *const RawMediaStreamTrack);
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaStreamTrackKind {
    Video,
    Audio,
}

#[repr(C)]
pub(crate) struct RawMediaStreamTrack {
    /// Returns a string set to "audio" if the track is an audio track and to
    /// "video", if it is a video track. It doesn't change if the track is
    /// disassociated from its source.
    pub kind: MediaStreamTrackKind,
    /// Returns a string containing a user agent-assigned label that identifies
    /// the track source, as in "internal microphone". The string may be
    /// left empty and is empty as long as no source has been connected.
    /// When the track is disassociated from its source, the label is not
    /// changed.
    pub label: *const c_char,
    /// Returns a Boolean with a value of true if the track is sourced by a
    /// RTCPeerConnection, false otherwise.
    pub remote: bool,

    // video
    video_source: *const c_void,
    video_sink: *const c_void,

    // audio
    audio_sink: *const c_void,
}

/// The MediaStreamTrack interface represents a single media track within
/// a stream typically.
///
/// these are audio or video tracks, but other track types may exist as
/// well.
#[derive(Debug, Clone)]
pub enum MediaStreamTrack {
    Audio(Arc<AudioTrack>),
    Video(Arc<VideoTrack>),
}

impl MediaStreamTrack {
    pub fn from_video(track: Arc<VideoTrack>) -> Arc<Self> {
        Arc::new(Self::Video(track))
    }

    /// Created through the original media stream track, video and audio
    /// are processed separately.
    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Self {
        assert!(!raw.is_null());
        match unsafe { (&*raw).kind } {
            MediaStreamTrackKind::Audio => {
                Self::Audio(AudioTrack::from_raw(raw))
            },
            MediaStreamTrackKind::Video => {
                Self::Video(VideoTrack::from_raw(raw))
            },
        }
    }

    /// get raw media stream track ptr.
    pub(crate) fn get_raw(&self) -> *const RawMediaStreamTrack {
        match self {
            Self::Audio(track) => track.raw,
            Self::Video(track) => track.raw,
        }
    }
}

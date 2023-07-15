use std::{
    ffi::{c_char, c_void},
    sync::Arc,
};

use anyhow::Result;

use crate::{AudioTrack, VideoTrack};

extern "C" {
    pub(crate) fn rtc_free_frame(frame: *const c_void);
    pub(crate) fn rtc_remove_media_stream_track_frame_h(
        track: *const crate::media_stream_track::RawMediaStreamTrack,
    );

    pub(crate) fn rtc_free_media_stream_track(
        track: *const crate::media_stream_track::RawMediaStreamTrack,
    );
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
#[derive(Clone)]
pub enum MediaStreamTrack {
    Audio(Arc<AudioTrack>),
    Video(Arc<VideoTrack>),
}

impl MediaStreamTrack {
    pub fn create_video_track(label: &str) -> Result<Self> {
        Ok(Self::Video(VideoTrack::new(label)?))
    }

    pub fn create_audio_track(label: &str) -> Result<Self> {
        Ok(Self::Audio(AudioTrack::new(label)?))
    }

    /// Created through the original media stream track, video and audio
    /// are processed separately.
    pub(crate) fn from_raw(raw: *const RawMediaStreamTrack) -> Self {
        assert!(!raw.is_null());
        match unsafe { (*raw).kind } {
            MediaStreamTrackKind::Audio => Self::Audio(AudioTrack::from_raw(raw)),
            MediaStreamTrackKind::Video => Self::Video(VideoTrack::from_raw(raw)),
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

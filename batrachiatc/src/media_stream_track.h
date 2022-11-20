#pragma once

#include "media/base/adapted_video_track_source.h"
#include "platform.h"

typedef struct
{
    uint32_t width;
    uint32_t height;

    uint8_t* data_y;
    uint32_t stride_y;
    uint8_t* data_u;
    uint32_t stride_u;
    uint8_t* data_v;
    uint32_t stride_v;
    
    bool remote;
} I420Frame;

class IVideoSourceTrack
    : public rtc::AdaptedVideoTrackSource
{
public:
    IVideoSourceTrack(std::string id);
    static IVideoSourceTrack* Create(std::string id);
    void AddFrame(I420Frame* frame);
    bool remote() const;
    bool is_screencast() const;
    webrtc::MediaSourceInterface::SourceState state() const;
    absl::optional<bool> needs_denoising() const;
    
    std::string id;
};

class IVideoSinkTrack
    : public rtc::VideoSinkInterface<webrtc::VideoFrame>
    , public rtc::RefCountInterface
{
public:
    IVideoSinkTrack(webrtc::VideoTrackInterface* track);
    static IVideoSinkTrack* Create(webrtc::VideoTrackInterface* track);
    void OnFrame(const webrtc::VideoFrame& frame);
    void SetOnFrame(void* ctx, void(*handler)(void* ctx, I420Frame* frame));
private:
    void(*_on_frame)(void* ctx, I420Frame* frame) = NULL;
    webrtc::VideoTrackInterface* _track;
    rtc::VideoSinkWants _wants;
    void* _ctx;
};

typedef enum {
    MediaStreamTrackKindVideo,
    MediaStreamTrackKindAudio,
} MediaStreamTrackKind;

/*
MediaStreamTrack

The MediaStreamTrack interface represents a single media track within a stream;
typically, these are audio or video tracks, but other track types may exist as
well.
*/
typedef struct {
    /*
    Returns a string set to "audio" if the track is an audio track and to
    "video", if it is a video track. It doesn't change if the track is
    disassociated from its source.
    */
    MediaStreamTrackKind kind;
    /*
    Returns a string containing a user agent-assigned label that identifies the
    track source, as in "internal microphone". The string may be left empty and
    is empty as long as no source has been connected. When the track is
    disassociated from its source, the label is not changed.
    */
    char* label;
    /*
    Returns a Boolean with a value of true if the track is sourced by a
    RTCPeerConnection, false otherwise.
    */
    bool remote;

    /* --------------- video --------------- */
    IVideoSourceTrack* video_source;
    IVideoSinkTrack* video_sink;

    /* --------------- audio --------------- */
} MediaStreamTrack;

extern "C" EXPORT void media_stream_video_track_add_frame(
    MediaStreamTrack* track, 
    I420Frame* frame);

extern "C" EXPORT void media_stream_video_track_on_frame(
    MediaStreamTrack * track,
    void(handler)(void* ctx, I420Frame * frame),
    void* ctx);

extern "C" EXPORT MediaStreamTrack* create_media_stream_video_track(
    char* id,
    char* label);

extern "C" EXPORT void free_i420_frame(I420Frame * frame);
extern "C" EXPORT void free_media_track(MediaStreamTrack * track);

MediaStreamTrack* media_stream_video_track_from(
    webrtc::VideoTrackInterface* track);

I420Frame* into_c(webrtc::VideoFrame* frame);
webrtc::VideoFrame from_c(I420Frame* frame);
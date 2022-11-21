#pragma once

#include "media/base/adapted_video_track_source.h"
#include "rtc_base/synchronization/mutex.h"
#include "media/base/video_broadcaster.h"
#include "api/video/video_frame_buffer.h"
#include "media/base/video_adapter.h"
#include "pc/video_track_source.h"
#include "api/video/i420_buffer.h"

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

typedef struct
{
    int cropped_width = 0;
    int cropped_height = 0;
    int width = 0;
    int height = 0;
    bool drop;
    bool resize;
} AdaptFrameResult;

class FramePreprocessor
{
public:
    virtual ~FramePreprocessor() = default;
    virtual webrtc::VideoFrame Preprocess(const webrtc::VideoFrame& frame) = 0;
};

class IVideoSourceTrack
    : public rtc::VideoSourceInterface<webrtc::VideoFrame>
    , public webrtc::VideoTrackSource
{
public:
    IVideoSourceTrack(): VideoTrackSource(false) {}
    static IVideoSourceTrack* Create();
    void AddOrUpdateSink(rtc::VideoSinkInterface<webrtc::VideoFrame>* sink, const rtc::VideoSinkWants& wants);
    void RemoveSink(rtc::VideoSinkInterface<webrtc::VideoFrame>* sink);
    void AddFrame(const webrtc::VideoFrame& original_frame);
    rtc::VideoSourceInterface<webrtc::VideoFrame>* source();
private:
    webrtc::VideoFrame _MaybePreprocess(const webrtc::VideoFrame& frame);
    AdaptFrameResult _AdaptFrameResolution(const webrtc::VideoFrame& frame);
    webrtc::VideoFrame _ScaleFrame(const webrtc::VideoFrame& original_frame, AdaptFrameResult& ret);

    webrtc::Mutex _lock;
    rtc::VideoBroadcaster _broadcaster;
    cricket::VideoAdapter _video_adapter;
    std::unique_ptr<FramePreprocessor> _preprocessor RTC_GUARDED_BY(_lock);
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

extern "C" void media_stream_video_track_on_frame(
    MediaStreamTrack * track,
    void(handler)(void* ctx, I420Frame * frame),
    void* ctx);
extern "C" void media_stream_video_track_add_frame(MediaStreamTrack * track, I420Frame * frame);
extern "C" MediaStreamTrack* create_media_stream_video_track(char* label);
extern "C" void free_media_track(MediaStreamTrack * track);
extern "C" void free_i420_frame(I420Frame * frame);

MediaStreamTrack* media_stream_video_track_from(webrtc::VideoTrackInterface* track);
I420Frame* into_c(webrtc::VideoFrame* frame);
webrtc::VideoFrame from_c(I420Frame* frame);
#ifndef BATRACHIATC_MEDIA_STREAM_TRACK_H_
#define BATRACHIATC_MEDIA_STREAM_TRACK_H_
#pragma once

#include "rtc_base/synchronization/mutex.h"
#include "media/base/video_broadcaster.h"
#include "api/video/video_frame_buffer.h"
#include "media/base/video_adapter.h"
#include "pc/video_track_source.h"
#include "api/video/i420_buffer.h"
#include "base.h"

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
    const uint8_t* buf;
    int bits_per_sample;
    int sample_rate;
    int channels;
    int frames;
} PCMFrames;

/*
Video source
*/

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

class IVideoSource
    : public rtc::VideoSourceInterface<webrtc::VideoFrame>
{
public:
    void AddOrUpdateSink(rtc::VideoSinkInterface<webrtc::VideoFrame>* sink, const rtc::VideoSinkWants& wants);
    void RemoveSink(rtc::VideoSinkInterface<webrtc::VideoFrame>* sink);
    void AddFrame(const webrtc::VideoFrame& original_frame);
private:
    webrtc::VideoFrame _MaybePreprocess(const webrtc::VideoFrame& frame);
    AdaptFrameResult _AdaptFrameResolution(const webrtc::VideoFrame& frame);
    webrtc::VideoFrame _ScaleFrame(const webrtc::VideoFrame& original_frame, AdaptFrameResult& ret);

    webrtc::Mutex _lock;
    rtc::VideoBroadcaster _broadcaster;
    cricket::VideoAdapter _video_adapter;
    std::unique_ptr<FramePreprocessor> _preprocessor RTC_GUARDED_BY(_lock);
};

class IVideoTrackSource
    : public webrtc::VideoTrackSource
{
public:
    IVideoTrackSource(): VideoTrackSource(false) {}
    static IVideoTrackSource* Create();
    void AddFrame(const webrtc::VideoFrame& frame);
    rtc::VideoSourceInterface<webrtc::VideoFrame>* source();
private:
    IVideoSource _source;
};

/*
Video Sink
*/

class IVideoTrackSink
    : public rtc::VideoSinkInterface<webrtc::VideoFrame>
    , public rtc::RefCountInterface
{
public:
    IVideoTrackSink(webrtc::VideoTrackInterface* track);
    static IVideoTrackSink* Create(webrtc::VideoTrackInterface* track);
    void OnFrame(const webrtc::VideoFrame& frame);
    void SetOnFrame(void* ctx, void(*handler)(void* ctx, I420Frame* frame));
private:
    void(*_on_frame)(void* ctx, I420Frame* frame) = NULL;
    webrtc::VideoTrackInterface* _track;
    rtc::VideoSinkWants _wants;
    void* _ctx;
};

/*
audio sink
*/

class IAudioTrackSink
    : public webrtc::AudioTrackSinkInterface
    , public rtc::RefCountInterface
{
public:
    IAudioTrackSink(webrtc::AudioTrackInterface* track);
    static IAudioTrackSink* Create(webrtc::AudioTrackInterface* track);
    void OnData(const void* buf, int b, int s, size_t c, size_t f);
    void SetOnFrame(void* ctx, void(*handler)(void* ctx, PCMFrames* frame));
private:
    void(*_handler)(void* ctx, PCMFrames* frame) = NULL;
    webrtc::AudioTrackInterface* _track = NULL;
    void* _ctx = NULL;
};

/*
extern
*/

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
    IVideoTrackSource* video_source;
    IVideoTrackSink* video_sink;

    /* --------------- audio --------------- */
    IAudioTrackSink* audio_sink;
} MediaStreamTrack;

extern "C" EXPORT void media_stream_video_track_on_frame(
    MediaStreamTrack * track,
    void(handler)(void* ctx, I420Frame * frame),
    void* ctx);
extern "C" EXPORT void media_stream_audio_track_on_frame(
    MediaStreamTrack* track,
    void(handler)(void* ctx, PCMFrames* frame),
    void* ctx);
extern "C" EXPORT void media_stream_video_track_add_frame(MediaStreamTrack * track, I420Frame* frame);
extern "C" EXPORT MediaStreamTrack* create_media_stream_video_track(char* label);
extern "C" EXPORT void free_media_track(MediaStreamTrack * track);
extern "C" EXPORT void free_i420_frame(I420Frame* frame);
extern "C" EXPORT void free_pcm_frames(PCMFrames* frame);

MediaStreamTrack* media_stream_video_track_from(webrtc::VideoTrackInterface* track);
MediaStreamTrack* media_stream_audio_track_from(webrtc::AudioTrackInterface* track);
PCMFrames* into_c(const uint8_t* buf, int b, int r, size_t c, size_t f);
I420Frame* into_c(webrtc::VideoFrame* frame);
webrtc::VideoFrame from_c(I420Frame* frame);

#endif  // BATRACHIATC_MEDIA_STREAM_TRACK_H_
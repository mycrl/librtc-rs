#ifndef BATRACHIATC_MEDIA_STREAM_TRACK_H_
#define BATRACHIATC_MEDIA_STREAM_TRACK_H_
#pragma once

#include "rtc_base/synchronization/mutex.h"
#include "media/base/video_broadcaster.h"
#include "api/video/video_frame_buffer.h"
#include "media/base/video_adapter.h"
#include "pc/video_track_source.h"
#include "api/audio/audio_mixer.h"
#include "frame.h"
#include "base.h"

/*
video source
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
video Sink
*/

class IVideoTrackSink
    : public rtc::VideoSinkInterface<webrtc::VideoFrame>
    , public rtc::RefCountInterface
{
public:
    IVideoTrackSink(webrtc::VideoTrackInterface* track);
    static IVideoTrackSink* Create(webrtc::VideoTrackInterface* track);
    void OnFrame(const webrtc::VideoFrame& frame);
    void SetOnFrame(void* ctx, void(*handler)(void* ctx, IVideoFrame* frame));
    void RemoveOnFrame();
private:
    void(*_handler)(void* ctx, IVideoFrame* frame) = NULL;
    webrtc::VideoTrackInterface* _track;
    rtc::VideoSinkWants _wants;
    void* _ctx;
};

/*
audio source
*/
class IAudioTrackSource
    : public webrtc::AudioMixer::Source
    , public rtc::RefCountInterface
{
public:
    IAudioTrackSource()
    {

    }

    static IAudioTrackSource* Create()
    {

    }

    AudioFrameInfo GetAudioFrameWithInfo(int sample_rate_hz,
        webrtc::AudioFrame* frame)
    {

    }

    int Ssrc() const
    {

    }

    int PreferredSampleRate() const
    {

    }
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
    void SetOnFrame(void* ctx, void(*handler)(void* ctx, IAudioFrame* frame));
    void RemoveOnFrame();
private:
    void(*_handler)(void* ctx, IAudioFrame* frame) = NULL;
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
    IAudioTrackSource* audio_source;
    IAudioTrackSink* audio_sink;
} MediaStreamTrack;

extern "C" EXPORT void media_stream_video_track_on_frame(
    MediaStreamTrack * track,
    void(handler)(void* ctx, IVideoFrame * frame),
    void* ctx);

extern "C" EXPORT void media_stream_audio_track_on_frame(
    MediaStreamTrack* track,
    void(handler)(void* ctx, IAudioFrame* frame),
    void* ctx);

extern "C" EXPORT void media_stream_video_track_add_frame(
    MediaStreamTrack * track, 
    IVideoFrame* frame);

extern "C" EXPORT void media_stream_track_stop_on_frame(
    MediaStreamTrack * track);

extern "C" EXPORT MediaStreamTrack* create_media_stream_video_track(char* label);
extern "C" EXPORT void free_media_track(MediaStreamTrack * track);

MediaStreamTrack* media_stream_video_track_from(webrtc::VideoTrackInterface* track);
MediaStreamTrack* media_stream_audio_track_from(webrtc::AudioTrackInterface* track);

#endif  // BATRACHIATC_MEDIA_STREAM_TRACK_H_
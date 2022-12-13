#pragma once

#include "api/video/i420_buffer.h"
#include "media_stream_track.h"

void free_media_track(MediaStreamTrack* track)
{
    if (track->remote)
    {
        free_incomplete_ptr(track->label);
    }

    free_incomplete_ptr(track);
}

/*
IVideoSource
*/
void IVideoSource::AddOrUpdateSink(
    rtc::VideoSinkInterface<webrtc::VideoFrame>* sink, 
    const rtc::VideoSinkWants& wants)
{
    _broadcaster.AddOrUpdateSink(sink, wants);
}

void IVideoSource::RemoveSink(
    rtc::VideoSinkInterface<webrtc::VideoFrame>* sink)
{
    _broadcaster.RemoveSink(sink);
}

void IVideoSource::AddFrame(const webrtc::VideoFrame& original_frame)
{
    auto frame = _MaybePreprocess(original_frame);
    auto ret = _AdaptFrameResolution(frame);
    if (!ret.drop)
    {
        return;
    }

    if (ret.resize)
    {
        _broadcaster.OnFrame(_ScaleFrame(frame, ret));
    }
    else
    {
        _broadcaster.OnFrame(frame);
    }
}

webrtc::VideoFrame IVideoSource::_MaybePreprocess(
    const webrtc::VideoFrame& frame)
{
    webrtc::MutexLock lock(&_lock);
    if (_preprocessor != nullptr)
    {
        return _preprocessor->Preprocess(frame);
    }
    else
    {
        return frame;
    }
}

webrtc::VideoFrame IVideoSource::_ScaleFrame(
    const webrtc::VideoFrame& original_frame, 
    AdaptFrameResult& ret)
{
    auto scaled_buffer = webrtc::I420Buffer::Create(ret.width, ret.height);
    scaled_buffer->ScaleFrom(*original_frame.video_frame_buffer()->ToI420());
    auto new_frame_builder = webrtc::VideoFrame::Builder()
        .set_video_frame_buffer(scaled_buffer)
        .set_rotation(webrtc::VideoRotation::kVideoRotation_0)
        .set_timestamp_us(original_frame.timestamp_us())
        .set_id(original_frame.id());
    
    if (!original_frame.has_update_rect())
    {
        return new_frame_builder.build();
    }

    auto rect = original_frame.update_rect().ScaleWithFrame(
        original_frame.width(),
        original_frame.height(),
        0,
        0,
        original_frame.width(),
        original_frame.height(),
        ret.width,
        ret.height);
    new_frame_builder.set_update_rect(rect);
    return new_frame_builder.build();
}

AdaptFrameResult IVideoSource::_AdaptFrameResolution(
    const webrtc::VideoFrame& frame)
{
    AdaptFrameResult ret;
    ret.drop = _video_adapter.AdaptFrameResolution(
        frame.width(),
        frame.height(),
        frame.timestamp_us() * 1000,
        &ret.cropped_width,
        &ret.cropped_height,
        &ret.width,
        &ret.height);
    ret.resize = ret.height != frame.height() || 
        ret.width != frame.width();
    return ret;
}

/*
IVideoTrackSource
*/

IVideoTrackSource* IVideoTrackSource::Create()
{
    auto self = new rtc::RefCountedObject<IVideoTrackSource>();
    self->AddRef();
    return self;
}

void IVideoTrackSource::AddFrame(const webrtc::VideoFrame& frame)
{
    _source.AddFrame(frame);
}

rtc::VideoSourceInterface<webrtc::VideoFrame>* IVideoTrackSource::source()
{
    return static_cast<rtc::VideoSourceInterface<webrtc::VideoFrame>*>(&_source);
}

/*
IVideoTrackSink
*/

IVideoTrackSink::IVideoTrackSink(webrtc::VideoTrackInterface* track)
{
    _ctx = NULL;
    _track = track;
    _track->AddRef();
}

IVideoTrackSink* IVideoTrackSink::Create(webrtc::VideoTrackInterface* track)
{
    auto self = new rtc::RefCountedObject<IVideoTrackSink>(track);
    self->AddRef();
    return self;
}

void IVideoTrackSink::OnFrame(const webrtc::VideoFrame& frame)
{
    if (!_handler)
    {
        return;
    }

    auto i420_frame = into_c((webrtc::VideoFrame*)&frame);
    if (!i420_frame)
    {
        return;
    }

    _handler(_ctx, i420_frame);
}

void IVideoTrackSink::SetOnFrame(void* ctx, 
    void(*handler)(void* ctx, IVideoFrame* frame))
{
    _track->AddOrUpdateSink(this, _wants);
    _handler = handler;
    _ctx = ctx;
}

void IVideoTrackSink::RemoveOnFrame()
{
    _track->RemoveSink(this);
    _handler = NULL;
    _ctx = NULL;
}

/*
IAudioTrackSink
*/

IAudioTrackSink::IAudioTrackSink(webrtc::AudioTrackInterface* track)
{
    _track = track;
    _track->AddRef();
}

IAudioTrackSink* IAudioTrackSink::Create(webrtc::AudioTrackInterface* track)
{
    auto self = new rtc::RefCountedObject<IAudioTrackSink>(track);
    self->AddRef();
    return self;
}

void IAudioTrackSink::OnData(const void* audio_data,
    int bits_per_sample,
    int sample_rate,
    size_t number_of_channels,
    size_t number_of_frames)
{
    if (!_handler || !audio_data)
    {
        return;
    }

    auto frames = into_c((const uint8_t*)audio_data,
        bits_per_sample,
        sample_rate,
        number_of_channels,
        number_of_frames);
    _handler(_ctx, frames);
}

void IAudioTrackSink::SetOnFrame(void* ctx, 
    void(*handler)(void* ctx, IAudioFrame* frame))
{
    _track->AddSink(this);
    _handler = handler;
    _ctx = ctx;
}

void IAudioTrackSink::RemoveOnFrame()
{
    _track->RemoveSink(this);
    _handler = NULL;
    _ctx = NULL;
}

/*
extern
*/

void media_stream_video_track_add_frame(MediaStreamTrack* track, IVideoFrame* frame)
{
    if (!track->video_source) {
        return;
    }

    track->video_source->AddFrame(from_c(frame));
}

void media_stream_video_track_on_frame(
    MediaStreamTrack* track,
    void(handler)(void* ctx, IVideoFrame* frame),
    void* ctx)
{
    if (!track->video_sink) {
        return;
    }

    track->video_sink->SetOnFrame(ctx, handler);
}

MediaStreamTrack* create_media_stream_video_track(char* label)
{
    MediaStreamTrack* track = (MediaStreamTrack*)malloc(sizeof(MediaStreamTrack));
    if (!track)
    {
        return NULL;
    }

    track->video_source = IVideoTrackSource::Create();
    if (!track->video_source)
    {
        return NULL;
    }

    track->kind = MediaStreamTrackKindVideo;
    track->remote = false;
    track->label = label;

    return track;
}

MediaStreamTrack* media_stream_video_track_from(webrtc::VideoTrackInterface* itrack)
{
    MediaStreamTrack* track = (MediaStreamTrack*)malloc(sizeof(MediaStreamTrack));
    if (!track)
    { 
        return NULL;
    }

    track->video_sink = IVideoTrackSink::Create(itrack);
    if (!track->video_sink)
    {
        free_media_track(track);
        return NULL;
    }

    auto id = itrack->id();
    track->label = (char*)malloc(sizeof(char) * id.size() + 1);
    if (!track->label)
    {
        free_media_track(track);
        return NULL;
    }

    strcpy(track->label, id.c_str());

    track->kind = MediaStreamTrackKindVideo;
    track->remote = true;

    return track;
}

MediaStreamTrack* media_stream_audio_track_from(webrtc::AudioTrackInterface* itrack)
{
    MediaStreamTrack* track = (MediaStreamTrack*)malloc(sizeof(MediaStreamTrack));
    if (!track)
    {
        free_media_track(track);
        return NULL;
    }

    track->audio_sink = IAudioTrackSink::Create(itrack);
    if (!track->audio_sink)
    {
        free_media_track(track);
        return NULL;
    }

    auto id = itrack->id();
    track->label = (char*)malloc(sizeof(char) * id.size() + 1);
    if (!track->label)
    {
        free_media_track(track);
        return NULL;
    }

    strcpy(track->label, id.c_str());

    track->kind = MediaStreamTrackKindAudio;
    track->remote = true;

    return track;
}

void media_stream_audio_track_on_frame(
    MediaStreamTrack* track,
    void(handler)(void* ctx, IAudioFrame* frame),
    void* ctx)
{
    track->audio_sink->SetOnFrame(ctx, handler);
}

void media_stream_track_stop_on_frame(MediaStreamTrack* track)
{
    if (track->video_sink)
    {
        track->video_sink->RemoveOnFrame();
    }

    if (track->audio_sink)
    {
        track->audio_sink->RemoveOnFrame();
    }
}
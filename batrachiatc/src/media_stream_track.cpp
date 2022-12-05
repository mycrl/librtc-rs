#pragma once

#include "api/video/i420_buffer.h"
#include "media_stream_track.h"
#include "base.h"

void free_i420_frame(I420Frame* frame)
{
    free_incomplete_ptr(frame->data_y);
    free_incomplete_ptr(frame->data_u);
    free_incomplete_ptr(frame->data_v);
    free_incomplete_ptr(frame);
}

void free_media_track(MediaStreamTrack* track)
{
    if (track->remote)
    {
        free_incomplete_ptr(track->label);
    }

    free_incomplete_ptr(track);
}

void free_pcm_frames(PCMFrames* frames)
{
    free_incomplete_ptr(frames);
}

I420Frame* into_c(webrtc::VideoFrame* frame)
{
    I420Frame* i420_frame = (I420Frame*)malloc(sizeof(I420Frame));
    if (!i420_frame)
    {
        free_i420_frame(i420_frame);
        return NULL;
    }

    auto video_frame_buf = frame->video_frame_buffer();
    auto i420_buf = video_frame_buf->GetI420();
    if (!i420_buf)
    {
        i420_buf = video_frame_buf->ToI420().get();
    }

    i420_frame->remote = true;

    i420_frame->stride_y = i420_buf->StrideY();
    i420_frame->stride_u = i420_buf->StrideU();
    i420_frame->stride_v = i420_buf->StrideV();

    i420_frame->width = i420_buf->width();
    i420_frame->height = i420_buf->height();

    int size_y = i420_frame->stride_y * i420_frame->height;
    i420_frame->data_y = (uint8_t*)malloc(sizeof(uint8_t) * size_y);
    if (!i420_frame->data_y)
    {
        free_i420_frame(i420_frame);
        return NULL;
    }

    int size_u = i420_frame->stride_u * (i420_frame->height / 2);
    i420_frame->data_u = (uint8_t*)malloc(sizeof(uint8_t) * size_u);
    if (!i420_frame->data_u)
    {
        free_i420_frame(i420_frame);
        return NULL;
    }

    int size_v = i420_frame->stride_v * (i420_frame->height / 2);
    i420_frame->data_v = (uint8_t*)malloc(sizeof(uint8_t) * size_v);
    if (!i420_frame->data_v)
    {
        free_i420_frame(i420_frame);
        return NULL;
    }

    memcpy(i420_frame->data_y, i420_buf->DataY(), size_y);
    memcpy(i420_frame->data_u, i420_buf->DataU(), size_u);
    memcpy(i420_frame->data_v, i420_buf->DataV(), size_v);

    return i420_frame;
}

webrtc::VideoFrame from_c(I420Frame* frame)
{
    auto i420_buf = webrtc::I420Buffer::Copy(
        frame->width, frame->height,
        frame->data_y, frame->stride_y,
        frame->data_u, frame->stride_u,
        frame->data_v, frame->stride_v);
    return webrtc::VideoFrame(i420_buf, 0, 0, webrtc::kVideoRotation_0);
}

PCMFrames* into_c(const uint8_t* buf,
    int bits_per_sample,
    int sample_rate,
    size_t channels,
    size_t frames_)
{
    PCMFrames* frames = (PCMFrames*)malloc(sizeof(PCMFrames));
    if (!frames)
    {
        return NULL;
    }

    frames->buf = buf;
    frames->frames = frames_;
    frames->channels = channels;
    frames->sample_rate = sample_rate;
    frames->bits_per_sample = bits_per_sample;
    return frames;
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
    _track->AddOrUpdateSink(this, _wants);
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
    if (!_on_frame)
    {
        return;
    }

    auto i420_frame = into_c((webrtc::VideoFrame*)&frame);
    if (!i420_frame)
    {
        return;
    }

    _on_frame(_ctx, i420_frame);
}

void IVideoTrackSink::SetOnFrame(void* ctx, 
    void(*handler)(void* ctx, I420Frame* frame))
{
    _on_frame = handler;
    _ctx = ctx;
}

/*
IAudioTrackSink
*/

IAudioTrackSink::IAudioTrackSink(webrtc::AudioTrackInterface* track)
{
    _track = track;
    _track->AddSink(this);
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
    if (!_handler)
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
    void(*handler)(void* ctx, PCMFrames* frame))
{
    _handler = handler;
    _ctx = ctx;
}

/*
extern
*/

void media_stream_video_track_add_frame(MediaStreamTrack* track, I420Frame* frame)
{
    if (!track->video_source) {
        return;
    }

    track->video_source->AddFrame(from_c(frame));
}

void media_stream_video_track_on_frame(
    MediaStreamTrack* track,
    void(handler)(void* ctx, I420Frame* frame),
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
        free_media_track(track);
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
    void(handler)(void* ctx, PCMFrames* frame),
    void* ctx)
{
    track->audio_sink->SetOnFrame(ctx, handler);
}
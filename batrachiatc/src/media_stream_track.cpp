#pragma once

#include "media_stream_track.h"
#include "api/video/i420_buffer.h"

IVideoSourceTrack::IVideoSourceTrack(std::string id_)
{
    id = id_;
}

IVideoSourceTrack* IVideoSourceTrack::Create(std::string id)
{
    auto self = new rtc::RefCountedObject<IVideoSourceTrack>(id);
    self->AddRef();
    return self;
}

void IVideoSourceTrack::AddFrame(I420Frame* frame)
{
    auto i420_buf = webrtc::I420Buffer::Copy(
        frame->width, frame->height,
        frame->data_y, frame->stride_y,
        frame->data_u, frame->stride_u,
        frame->data_v, frame->stride_v);
    OnFrame(webrtc::VideoFrame(i420_buf, 0, 0, webrtc::kVideoRotation_0));
}

bool IVideoSourceTrack::remote() const
{
    return false;
}

bool IVideoSourceTrack::is_screencast() const
{
    return false;
}

webrtc::MediaSourceInterface::SourceState IVideoSourceTrack::state() const
{
    return webrtc::MediaSourceInterface::kLive;
}

absl::optional<bool> IVideoSourceTrack::needs_denoising() const
{
    return true;
}

void media_stream_video_track_add_frame(MediaStreamTrack* track, I420Frame* frame)
{
    track->video_track->AddFrame(frame);
}

MediaStreamTrack* create_media_stream_video_track(
    char* id,
    char* label,
    uint32_t width,
    uint32_t height,
    uint16_t frame_rate)
{
    MediaStreamTrack* track = (MediaStreamTrack*)malloc(sizeof(MediaStreamTrack));
    if (!track)
    {
        return NULL;
    }

    track->video_track = IVideoSourceTrack::Create(std::string(id));
    if (!track->video_track)
    {
        return NULL;
    }

    track->kind = MediaStreamTrackKindVideo;
    track->frame_rate = frame_rate;
    track->height = height;
    track->remote = false;
    track->width = width;
    track->label = label;
    track->id = id;

    return track;
}
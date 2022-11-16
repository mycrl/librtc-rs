#pragma once

#include "sys.h"
#include "media/base/video_broadcaster.h"
#include "media/base/adapted_video_track_source.h"
#include "api/video/i420_buffer.h"

typedef struct
{
    int width;
    int height;

    uint8_t* data_y;
    int stride_y;
    uint8_t* data_u;
    int stride_u;
    uint8_t* data_v;
    int stride_v;
} I420Frame;

class IVideoSourceTrack
    : public rtc::AdaptedVideoTrackSource
{
public:
    IVideoSourceTrack(std::string id_)
    {
        id = id_;
    }

    static rtc::scoped_refptr<IVideoSourceTrack> Create(std::string id)
    {
        return new rtc::RefCountedObject<IVideoSourceTrack>(id);
    }

    void AddFrame(I420Frame* frame)
    {
        auto i420_buf = webrtc::I420Buffer::Copy(
            frame->width, frame->height,
            frame->data_y, frame->stride_y,
            frame->data_u, frame->stride_u,
            frame->data_v, frame->stride_v);
        OnFrame(webrtc::VideoFrame(i420_buf, 0, 0, webrtc::kVideoRotation_0));
    }

    bool remote() const
    {
        return false;
    }

    bool is_screencast() const
    {
        return false;
    }

    webrtc::MediaSourceInterface::SourceState state() const
    {
        return webrtc::MediaSourceInterface::kLive;
    }

    absl::optional<bool> needs_denoising() const
    {
        return true;
    }

    std::string id;
};

typedef struct
{
    IVideoSourceTrack* track;
} MediaStreamTrack;

MediaStreamTrack* create_media_stream_track(char* id)
{
    MediaStreamTrack* media_stream_track = (MediaStreamTrack*)malloc(sizeof(MediaStreamTrack));
    if (!media_stream_track)
    {
        return NULL;
    }

    media_stream_track->track = IVideoSourceTrack::Create(std::string(id));
    return media_stream_track;
}

class IVideoSource
{
public:
    IVideoSource(std::string id_)
    {
        id = id_;
    }

    static rtc::scoped_refptr<IVideoSource> Create(std::string id)
    {
        return new rtc::RefCountedObject<IVideoSource>(id);
    }

    void AddTrack(IVideoSourceTrack* track)
    {
        _tracks.push_back(track);
        track->AddRef();
    }

    std::vector<IVideoSourceTrack*> GetTracks()
    {
        return _tracks;
    }

    std::string id;
private:
    std::vector<IVideoSourceTrack*> _tracks;
};

typedef struct
{
    IVideoSource* source;
} MediaStream;

MediaStream* create_media_stream(char* id)
{
    MediaStream* media_stream = (MediaStream*)malloc(sizeof(MediaStream));
    if (!media_stream)
    {
        return NULL;
    }

    media_stream->source = IVideoSource::Create(std::string(id));
    return media_stream;
}

void media_stream_add_track(MediaStream* media_stram, MediaStreamTrack* track)
{
    media_stram->source->AddTrack(track->track);
}

void media_stream_track_add_frame(MediaStreamTrack* track, I420Frame* frame)
{
    track->track->AddFrame(frame);
}
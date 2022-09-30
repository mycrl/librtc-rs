#pragma once

#include "sys.h"
#include "media/base/video_broadcaster.h"
#include "media/base/adapted_video_track_source.h"
#include "api/video/i420_buffer.h"

class VideoSource 
    : public rtc::AdaptedVideoTrackSource
    , public rtc::RefCountInterface
{
public:
    static VideoSource* Create()
    {
        new rtc::RefCountedObject<VideoSource>();
    }

    bool remote() const;
    bool is_screencast() const;
    webrtc::MediaSourceInterface::SourceState state() const;
    absl::optional<bool> needs_denoising() const;
};

bool VideoSource::remote() const 
{
    return false;
}

bool VideoSource::is_screencast() const
{
    return false;
}

absl::optional<bool> VideoSource::needs_denoising() const
{
    return false;
}

webrtc::MediaSourceInterface::SourceState VideoSource::state() const
{
    return SourceState::kInitializing;
}

typedef struct
{
    rtc::scoped_refptr<rtc::AdaptedVideoTrackSource> source;
    rtc::scoped_refptr<webrtc::I420Buffer> i420_buf;
    webrtc::VideoFrame* frame;
} IVideoSource;

typedef struct
{
    uint32_t width;
    uint32_t height;
    uint8_t* buffer;
} IVideoFrame;

typedef struct
{
    uint32_t width;
    uint32_t height;
} VideoDescripts;

IVideoSource* media_create_video_source(VideoDescripts* vdesc)
{
    IVideoSource* vs = (IVideoSource*)malloc(sizeof(IVideoSource));
    if (vs == NULL)
    {
        return NULL;
    }

    vs->source = VideoSource::Create();
    vs->i420_buf = webrtc::I420Buffer::Create(vdesc->width, vdesc->height);
    vs->frame = webrtc::VideoFrame(vs->i420_buf, 0, 0);
    return vs;
}

void media_video_source_add_frame(IVideoSource* vs,
    IVideoFrame* frame)
{
    // vs->source.get()->OnFrame();
}

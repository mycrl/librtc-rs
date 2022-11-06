#pragma once

#include "sys.h"
#include "media/base/video_broadcaster.h"
#include "media/base/adapted_video_track_source.h"
#include "api/video/i420_buffer.h"

typedef struct
{
    rtc::scoped_refptr<VideoSource> source;
} IVideoSource;

typedef struct
{
    uint32_t width;
    uint32_t height;
    uint8_t* buffer;
} IVideoFrame;

class VideoSource 
    : public rtc::AdaptedVideoTrackSource
    , public rtc::RefCountInterface
{
public:
    static VideoSource* Create();
    void PushFrame(webrtc::VideoFrameBuffer* buf);

    bool remote() const;
    bool is_screencast() const;
    webrtc::MediaSourceInterface::SourceState state() const;
    absl::optional<bool> needs_denoising() const;
};

VideoSource* VideoSource::Create()
{
    new rtc::RefCountedObject<VideoSource>();
}

void VideoSource::PushFrame(webrtc::VideoFrameBuffer* buf)
{
    this->OnFrame(webrtc::VideoFrame(buf, 0, 0, webrtc::kVideoRotation_0));
}

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

IVideoSource* media_create_video_source()
{
    IVideoSource* vs = (IVideoSource*)malloc(sizeof(IVideoSource));
    if (vs == NULL)
    {
        return NULL;
    }

    vs->source = VideoSource::Create();
    return vs;
}

void media_video_source_add_frame(IVideoSource* vs, IVideoFrame* frame)
{
    auto i420_buf = webrtc::I420Buffer::Copy(frame->width, frame->height, 
        frame->buffer, frame->width,
        frame->buffer);
    vs->source.get()->PushFrame(i420_buf);
}

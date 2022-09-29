#pragma once

#include "sys.h"
#include "media/base/video_broadcaster.h"
#include "media/base/adapted_video_track_source.h"

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
    SourceState state() const;
    absl::optional<bool> needs_denoising() const;
};

bool VideoSource::remote() const {
    return false;
}

typedef struct
{
    rtc::AdaptedVideoTrackSource* raw;
} IVideoSource;

typedef struct
{

} IVideoFrame;

IVideoSource* media_create_video_source()
{
    IVideoSource* vs = (IVideoSource*)malloc(sizeof(IVideoSource));
    if (vs == NULL)
    {
        return NULL;
    }

    vs->raw = VideoSource::Create();
    return vs;
}

void media_video_source_add_frame(IVideoSource* video_source,
    IVideoFrame* frame)
{

}

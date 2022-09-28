#pragma once

#include "sys.h"
#include "media/base/video_broadcaster.h"

class IVideoSource : public rtc::VideoSourceInterface<webrtc::VideoFrame>
{
public:
    void AddOrUpdateSink(rtc::VideoSinkInterface<webrtc::VideoFrame>* sink,
                               const rtc::VideoSinkWants& wants);
    void RemoveSink(rtc::VideoSinkInterface<webrtc::VideoFrame>* sink);
private:
    rtc::VideoSinkInterface<webrtc::VideoFrame>* _sink;
    rtc::VideoBroadcaster _broadcaster;
};

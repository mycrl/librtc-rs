#pragma once

#include "api/peer_connection_interface.h"
#include "media_stream_track.h"
#include "platform.h"

class IVideoSource
    : public rtc::RefCountInterface
{
public:
    IVideoSource(std::string id_);

    static IVideoSource* Create(std::string id);
    void AddTrack(MediaStreamTrack* track);
    std::vector<MediaStreamTrack*> GetTracks();
    
    std::string id;
private:
    std::vector<MediaStreamTrack*> _tracks;
};

/*
 The MediaStream() constructor returns a newly-created MediaStream, which serves as 
 a collection of media tracks, each represented by a MediaStreamTrack object.
 */
typedef struct
{
    IVideoSource* source;
} MediaStream;

typedef struct
{
    MediaStreamTrack** tracks;
    int len;
} MediaStreamTracks;

extern "C" EXPORT MediaStream* create_media_stream(char* id);
extern "C" EXPORT void media_stream_add_track(MediaStream* media_stram, MediaStreamTrack* track);
extern "C" EXPORT MediaStreamTracks media_stream_get_tracks(MediaStream* media_stram);
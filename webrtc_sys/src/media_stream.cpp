#pragma once

#include "media_stream.h"

IVideoSource::IVideoSource(std::string id_)
{
    id = id_;
}

IVideoSource* IVideoSource::Create(std::string id)
{
    auto self = new rtc::RefCountedObject<IVideoSource>(id);
    self->AddRef();
    return self;
}

void IVideoSource::AddTrack(MediaStreamTrack* track)
{
    _tracks.push_back(track);
    if (track->kind == "video")
    {
        track->video_track->track->AddRef();
    }
}

std::vector<MediaStreamTrack*> IVideoSource::GetTracks()
{
    return _tracks;
}

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
    media_stram->source->AddTrack(track);
}

MediaStreamTracks media_stream_get_tracks(MediaStream* media_stram)
{
    MediaStreamTracks tracks;
    auto vtracks = media_stram->source->GetTracks();
    tracks.tracks = *vtracks.data();
    tracks.len = vtracks.size();
    return tracks;
}
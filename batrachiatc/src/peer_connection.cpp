#include "api/audio_codecs/builtin_audio_decoder_factory.h"
#include "api/audio_codecs/builtin_audio_encoder_factory.h"
#include "api/video_codecs/builtin_video_decoder_factory.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"
#include "api/create_peerconnection_factory.h"
#include "rtc_base/ssl_adapter.h"
#include "peer_connection.h"

void rtc_run()
{
    rtc::InitializeSSL();
    rtc::Thread::Current()->Run();
}

void rtc_close(RTCPeerConnection* peer)
{
    peer->pc->Close();
    rtc::CleanupSSL();
    delete peer;
}

RTCPeerConnection* create_rtc_peer_connection(RTCPeerConnectionConfigure* c_config, 
    Events* events,
    void* ctx)
{
    RTCPeerConnection* rtc = new RTCPeerConnection();
    if (!rtc)
    {
        return NULL;
    }

    rtc->pc_factory = webrtc::CreatePeerConnectionFactory(
        nullptr,
        nullptr,
        nullptr,
        nullptr,
        webrtc::CreateBuiltinAudioEncoderFactory(),
        webrtc::CreateBuiltinAudioDecoderFactory(),
        webrtc::CreateBuiltinVideoEncoderFactory(),
        webrtc::CreateBuiltinVideoDecoderFactory(),
        nullptr,
        nullptr);
    if (!rtc->pc_factory)
    {
        return NULL;
    }

    rtc->pc = rtc->pc_factory->CreatePeerConnection(
        from_c(c_config),
        nullptr,
        nullptr,
        Observer::Create(events, ctx));
    if (!rtc->pc)
    {
        return NULL;
    }

    return rtc;
}

bool rtc_add_ice_candidate(RTCPeerConnection* rtc, RTCIceCandidate* icecandidate)
{
    return rtc->pc->AddIceCandidate(from_c(icecandidate));
}

void rtc_create_answer(RTCPeerConnection* rtc, CreateDescCallback callback, void* ctx)
{
    auto opt = webrtc::PeerConnectionInterface::RTCOfferAnswerOptions();
    rtc->pc->CreateAnswer(CreateDescObserver::Create(callback, ctx), opt);
}

void rtc_create_offer(RTCPeerConnection* rtc, CreateDescCallback callback, void* ctx)
{
    auto opt = webrtc::PeerConnectionInterface::RTCOfferAnswerOptions();
    rtc->pc->CreateOffer(CreateDescObserver::Create(callback, ctx), opt);
}

void rtc_set_local_description(RTCPeerConnection* rtc, 
    RTCSessionDescription* c_desc, 
    SetDescCallback callback, 
    void* ctx)
{
    auto observer = SetDescObserver::Create(callback, ctx);
    rtc->pc->SetLocalDescription(observer, from_c(c_desc).release());
}

void rtc_set_remote_description(RTCPeerConnection* rtc,
    RTCSessionDescription* c_desc,
    SetDescCallback callback,
    void* ctx)
{
    auto observer = SetDescObserver::Create(callback, ctx);
    rtc->pc->SetRemoteDescription(observer, from_c(c_desc).release());
}

void rtc_add_track(RTCPeerConnection* rtc, MediaStreamTrack* track, char* stream_id)
{
    if (track->kind == MediaStreamTrackKind::MediaStreamTrackKindVideo)
    {
        auto video_track = rtc->pc_factory->CreateVideoTrack(track->label, track->video_source);
        rtc->pc->AddTrack(video_track, { stream_id });
    }
}

RTCDataChannel* rtc_create_data_channel(RTCPeerConnection* rtc,
    char* label,
    DataChannelOptions* options)
{
    auto init = from_c(options);
    auto data_channel = rtc->pc->CreateDataChannel(std::string(label), init);
    return create_data_channel(data_channel);
}

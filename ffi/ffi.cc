#include "ffi.h"
#include "convert.h"
#include "observer.h"
#include "promisify.h"

#include "api/create_peerconnection_factory.h"
#include "api/peer_connection_interface.h"
#include "api/audio_codecs/builtin_audio_decoder_factory.h"
#include "api/audio_codecs/builtin_audio_encoder_factory.h"
#include "api/video_codecs/builtin_video_decoder_factory.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"

struct RTCPeerConnection* create_rtc_peer_connection(struct RTCPeerConnectionConfigure* c_config) 
{
    struct RTCPeerConnection* rtc = new RTCPeerConnection();
    auto peer_factory = webrtc::CreatePeerConnectionFactory(
        nullptr /* network_thread */,
        nullptr /* worker_thread */,
        nullptr /* signaling_thread */,
        nullptr /* default_adm */,
        webrtc::CreateBuiltinAudioEncoderFactory(),
        webrtc::CreateBuiltinAudioDecoderFactory(),
        webrtc::CreateBuiltinVideoEncoderFactory(),
        webrtc::CreateBuiltinVideoDecoderFactory(),
        nullptr /* audio_mixer */,
        nullptr /* audio_processing */
    );

    if (!peer_factory) 
    {
        return NULL;
    }

    rtc->observer = std::make_shared<Observer>();
    rtc->peer_connection = peer_factory->CreatePeerConnection(
        from_c(c_config),
        nullptr,
        nullptr,
        rtc->observer.get()
    );

    if (!rtc->peer_connection)
    {
        return NULL;
    }

    return rtc;
}

void rtc_add_ice_candidate(
    struct RTCPeerConnection* rtc, 
    struct RTCIceCandidate* icecandidate
)
{
    rtc->peer_connection->AddIceCandidate(from_c(icecandidate));
}

void rtc_free(struct RTCSessionDescription* raw)
{
    free((void*)raw->sdp);
    free(raw);
}

void rtc_create_answer(
    struct RTCPeerConnection* rtc,
    void* ctx,
    void (*callback)(struct RTCSessionDescription* desc, void* ctx)
)
{
    auto promisify = new rtc::RefCountedObject<CreateDescPromisify>(ctx, callback);
    auto options = webrtc::PeerConnectionInterface::RTCOfferAnswerOptions();
    rtc->peer_connection->CreateAnswer(promisify, options);
}

void rtc_create_offer(
    struct RTCPeerConnection* rtc,
    void* ctx,
    void (*callback)(struct RTCSessionDescription* desc, void* ctx)
)
{
    auto promisify = new rtc::RefCountedObject<CreateDescPromisify>(ctx, callback);
    auto options = webrtc::PeerConnectionInterface::RTCOfferAnswerOptions();
    rtc->peer_connection->CreateOffer(promisify, options);
}

void rtc_set_local_description(
    struct RTCPeerConnection* rtc, 
    struct RTCSessionDescription* c_desc, 
    void* ctx, 
    void (*callback)(int res, void* ctx)
)
{
    auto promisify = new rtc::RefCountedObject<SetDescPromisify>(ctx, callback);
    rtc->peer_connection->SetLocalDescription(promisify, from_c(c_desc));
}

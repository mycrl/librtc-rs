#include "ffi.h"
#include "convert.h"
#include "observer.h"

#include "api/create_peerconnection_factory.h"
#include "api/peer_connection_interface.h"
#include "api/audio_codecs/builtin_audio_decoder_factory.h"
#include "api/audio_codecs/builtin_audio_encoder_factory.h"
#include "api/video_codecs/builtin_video_decoder_factory.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"

struct RTCPeerConnection* create_rtc_peer_connection(struct RTCPeerConnectionConfigure* c_config) 
{
    rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> peer_factory = webrtc::CreatePeerConnectionFactory(
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

    if (!peer_factory) {
        return NULL;
    }

    struct RTCPeerConnection* peer = new RTCPeerConnection();

    peer->observer = std::make_shared<Observer>();
    peer->peer_connection = peer_factory->CreatePeerConnection(
        from_c(c_config),
        nullptr,
        nullptr,
        peer->observer.get()
    );

    if (!peer->peer_connection) {
        return NULL;
    }

    return peer;
}

void rtc_add_ice_candidate(struct RTCPeerConnection* peer, struct RTCIceCandidate icecandidate)
{
    peer->peer_connection.get()->AddIceCandidate(from_c(icecandidate));
}
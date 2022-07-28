#include "api/create_peerconnection_factory.h"
#include "api/peer_connection_interface.h"
#include "api/audio_codecs/builtin_audio_decoder_factory.h"
#include "api/audio_codecs/builtin_audio_encoder_factory.h"
#include "api/video_codecs/builtin_video_decoder_factory.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"
#include <iostream>

class PeerConnection : public webrtc::PeerConnectionObserver,
                       public webrtc::CreateSessionDescriptionObserver
{
private:
    rtc::scoped_refptr<webrtc::PeerConnectionInterface> _peer_connection;
    rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> _peer_connection_factory;
public:
    PeerConnection()
    {
        webrtc::PeerConnectionInterface::RTCConfiguration config;
        config.enable_dtls_srtp = true;
        config.sdp_semantics = webrtc::SdpSemantics::kUnifiedPlan;

        _peer_connection_factory = webrtc::CreatePeerConnectionFactory(
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
        if (!_peer_connection_factory)
        {
            throw "CreatePeerConnectionFactory failed!";
        }

        _peer_connection = _peer_connection_factory->CreatePeerConnection(
            config,
            nullptr,
            nullptr,
            this);
        if (!_peer_connection)
        {
            throw "CreatePeerConnection failed!";
        }

        _peer_connection->CreateOffer(
            this,
            webrtc::PeerConnectionInterface::RTCOfferAnswerOptions());
    }

    void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState new_state)
    {

    }

    void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel)
    {

    }

    void OnRenegotiationNeeded()
    {

    }

    void OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState new_state)
    {

    }

    void OnIceCandidate(const webrtc::IceCandidateInterface* candidate)
    {

    }

    void OnSuccess(webrtc::SessionDescriptionInterface* desc)
    {
        printf("OnSuccess");
    }

    void OnFailure(const std::string& error)
    {

    }
};

int main()
{
    auto peer = new rtc::RefCountedObject<PeerConnection>();
    Sleep(20000);
}
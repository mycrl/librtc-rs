#pragma once

#include "api/peer_connection_interface.h"
#include "sys.h"

class Observer
    : public webrtc::PeerConnectionObserver
    , public rtc::RefCountInterface
{
public:
    Observer(EventBus events);
    static Observer* Create(EventBus events);
    void OnSignalingChange(
        webrtc::PeerConnectionInterface::SignalingState state);
    void OnDataChannel(
        rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);
    void OnIceGatheringChange(
        webrtc::PeerConnectionInterface::IceGatheringState state);
    void OnIceCandidate(const webrtc::IceCandidateInterface* candidate);
    void OnRenegotiationNeeded();
    void OnIceConnectionChange(
        webrtc::PeerConnectionInterface::IceConnectionState state);
    void OnTrack(
        rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver);
    void OnConnectionChange(
        webrtc::PeerConnectionInterface::PeerConnectionState state);
private:
    EventBus _events;
};

class CreateDescObserver
    : public webrtc::CreateSessionDescriptionObserver
{
public:
    CreateDescObserver(CreateDescCallback callback, void* ctx);
    static CreateDescObserver* Create(
        CreateDescCallback callback, void* ctx);
    void OnSuccess(webrtc::SessionDescriptionInterface* desc);
    void OnFailure(webrtc::RTCError error);
private:
    CreateDescCallback _callback;
    void* _ctx;
};

class SetDescObserver
    : public webrtc::SetSessionDescriptionObserver 
{
public:
    SetDescObserver(SetDescCallback callback, void* ctx);
    static SetDescObserver* Create(SetDescCallback callback, void* ctx);
    virtual void OnSuccess();
    virtual void OnFailure(webrtc::RTCError error);
private:
    SetDescCallback _callback;
    void* _ctx;
};

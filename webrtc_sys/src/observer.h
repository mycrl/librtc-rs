#pragma once

#include "api/peer_connection_interface.h"
#include "sys.h"

class Observer
    : public webrtc::PeerConnectionObserver
    , public rtc::RefCountInterface
{
public:
    Observer(EventBus events);
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

class DummyCreateDescObserver
    : public webrtc::CreateSessionDescriptionObserver
{
public:
    DummyCreateDescObserver(CreateDescCallback callback, void* ctx);
    static DummyCreateDescObserver* Create(
        CreateDescCallback callback, void* ctx);
    void OnSuccess(webrtc::SessionDescriptionInterface* desc);
    void OnFailure(webrtc::RTCError error);
private:
    CreateDescCallback _callback;
    void* _ctx;
};

class DummySetDescObserver
    : public webrtc::SetSessionDescriptionObserver 
{
public:
    DummySetDescObserver(SetDescCallback callback, void* ctx);
    static DummySetDescObserver* Create(SetDescCallback callback, void* ctx);
    virtual void OnSuccess();
    virtual void OnFailure(webrtc::RTCError error);
private:
    SetDescCallback _callback;
    void* _ctx;
};

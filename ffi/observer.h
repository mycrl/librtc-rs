#pragma once

#include "api/peer_connection_interface.h"

class Observer: public webrtc::PeerConnectionObserver 
{
public:
    Observer() {}
    ~Observer() {}

    void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState new_state);
    void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);
    void OnRenegotiationNeeded();
    void OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState new_state);
    void OnIceCandidate(const webrtc::IceCandidateInterface* candidate);
public:
    void set_connectionstatechange_handler(void (*handler)(webrtc::PeerConnectionInterface::SignalingState new_state));
    void set_datachannel_handler(void (*handler)(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel));
    void set_renegotiationneeded_handler(void (*handler)());
    void set_icegatheringchange_handler(void (*handler)(webrtc::PeerConnectionInterface::IceGatheringState new_state));
    void set_icecandidate_handler(void (*handler)(const webrtc::IceCandidateInterface* candidate));
private:
    void (*_on_connectionstatechange_handler)(webrtc::PeerConnectionInterface::SignalingState new_state);
    void (*_on_datachannel_handler)(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);
    void (*_on_renegotiationneeded_handler)();
    void (*_on_icegatheringchange_handler)(webrtc::PeerConnectionInterface::IceGatheringState new_state);
    void (*_on_icecandidate_handler)(const webrtc::IceCandidateInterface* candidate);
};
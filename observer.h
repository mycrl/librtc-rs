#pragma once

#include "api/peer_connection_interface.h"

class Observer: public webrtc::PeerConnectionObserver {
public:
    Observer();
    ~Observer();

    void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState new_state);
    void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);
    void OnRenegotiationNeeded();
    void OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState new_state);
    void OnIceCandidate(const webrtc::IceCandidateInterface* candidate);
};
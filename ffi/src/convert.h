#pragma once

#include "ffi.h"
#include "api/peer_connection_interface.h"

/*
* c type to c++ type
*/
const std::string from_c(char* raw);
std::vector<std::string> from_c(char** raw, int size);
webrtc::PeerConnectionInterface::IceServer from_c(RTCIceServer raw);
webrtc::PeerConnectionInterface::IceServers from_c(RTCIceServer* raw, int size);
const webrtc::IceCandidateInterface* from_c(RTCIceCandidate* candidate);
webrtc::SessionDescriptionInterface* from_c(RTCSessionDescription* desc);
webrtc::PeerConnectionInterface::RTCConfiguration from_c(
    RTCPeerConnectionConfigure* raw);

/*
* c++ type to c type
*/
RTCSessionDescription* into_c(webrtc::SessionDescriptionInterface* raw);

void free_session_description(RTCSessionDescription * desc);

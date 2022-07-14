#pragma once

#include "ffi.h"
#include <string>
#include <vector>
#include "api/peer_connection_interface.h"

/*
* c type to c++ type
*/
std::string from_c(char* raw);
std::vector<std::string> from_c(struct Strings raw);
webrtc::PeerConnectionInterface::IceServer from_c(struct RTCIceServer raw);
webrtc::PeerConnectionInterface::IceServers from_c(struct RTCIceServers raw);
webrtc::PeerConnectionInterface::RTCConfiguration from_c(struct RTCPeerConnectionConfigure* raw);
const webrtc::IceCandidateInterface* from_c(struct RTCIceCandidate* candidate);
webrtc::SessionDescriptionInterface* from_c(struct RTCSessionDescription* desc);

/*
* c++ type to c type
*/
struct RTCSessionDescription* into_c(webrtc::SessionDescriptionInterface* raw);
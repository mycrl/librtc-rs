#pragma once

#include "ffi.h"
#include <string>
#include <vector>
#include "api/peer_connection_interface.h"

std::string from_c(char* raw);
std::vector<std::string> from_c(struct Strings raw);
webrtc::PeerConnectionInterface::IceServer from_c(struct RTCIceServer raw);
webrtc::PeerConnectionInterface::IceServers from_c(struct RTCIceServers raw);
webrtc::PeerConnectionInterface::RTCConfiguration from_c(struct RTCPeerConnectionConfigure* raw);
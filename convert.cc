#include "ffi.h"
#include <string>
#include <vector>
#include "api/peer_connection_interface.h"

std::string from_c(char* raw)
{
	return std::string(raw);
}

std::vector<std::string> from_c(struct Strings raw)
{
	std::vector<std::string> strings;
	for (int i = 0; i < raw.len; i++)
	{
		strings.push_back(from_c(raw.strs[i]));
	}
	
	return strings;
}

webrtc::PeerConnectionInterface::IceServer from_c(struct RTCIceServer raw)
{
	webrtc::PeerConnectionInterface::IceServer server;
	server.password = from_c(raw.credential);
	server.username = from_c(raw.username);
	server.urls = from_c(raw.urls);
	return server;
}

webrtc::PeerConnectionInterface::IceServers from_c(struct RTCIceServers raw) 
{
	webrtc::PeerConnectionInterface::IceServers servers;
	for (int i = 0; i < raw.len; i++)
	{
		servers.push_back(from_c(raw.servers[i]));
	}

	return servers;
}

webrtc::PeerConnectionInterface::RTCConfiguration from_c(struct RTCPeerConnectionConfigure* raw)
{
	webrtc::PeerConnectionInterface::RTCConfiguration config;
	config.sdp_semantics = webrtc::SdpSemantics::kUnifiedPlan;
	config.enable_dtls_srtp = true;
	config.type = (webrtc::PeerConnectionInterface::IceTransportsType)raw->ice_transport_policy;
	config.bundle_policy = (webrtc::PeerConnectionInterface::BundlePolicy)raw->bundle_policy;
	config.rtcp_mux_policy = (webrtc::PeerConnectionInterface::RtcpMuxPolicy)raw->rtcp_mux_policy;
	config.servers = from_c(raw->ice_servers);
	config.ice_candidate_pool_size = raw->ice_candidate_pool_size;
	return config;
}

const webrtc::IceCandidateInterface* from_c(struct RTCIceCandidate candidate)
{
	return webrtc::CreateIceCandidate(&from_c(candidate.sdp_Mid),
		candidate.sdp_mline_index,
		candidate.candidate);
}
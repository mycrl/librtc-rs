#include "peer_connection_config.h"
#include <vector>
#include <string>

/**
 * string array
 */

std::vector<std::string> from_c(char** raw, int size)
{
	std::vector<std::string> strings;
	for (int i = 0; i < size; i++)
	{
		strings.push_back(std::string(raw[i]));
	}

	return strings;
}

/**
 * rtc peerconnection config
 */

webrtc::PeerConnectionInterface::IceServer from_c(RTCIceServer raw)
{
	webrtc::PeerConnectionInterface::IceServer server;

	if (raw.credential)
	{
		server.password = std::string(raw.credential);
	}

	if (raw.username)
	{
		server.username = std::string(raw.username);
	}

	if (raw.urls)
	{
		server.urls = from_c(raw.urls, raw.urls_size);
	}

	return server;
}

webrtc::PeerConnectionInterface::IceServers from_c(RTCIceServer* raw, int size)
{
	webrtc::PeerConnectionInterface::IceServers servers;
	for (int i = 0; i < size; i++)
	{
		servers.push_back(from_c(raw[i]));
	}

	return servers;
}

webrtc::PeerConnectionInterface::RTCConfiguration from_c(
	RTCPeerConnectionConfigure* raw)
{
	using Peer = webrtc::PeerConnectionInterface;

	Peer::RTCConfiguration config;
	config.sdp_semantics = webrtc::SdpSemantics::kUnifiedPlan;

	if (!raw)
	{
		return config;
	}

	if (raw->ice_candidate_pool_size)
	{
		config.ice_candidate_pool_size = raw->ice_candidate_pool_size;
	}

	if (raw->ice_transport_policy)
	{
		config.type = (Peer::IceTransportsType)(raw->ice_transport_policy - 1);
	}

	if (raw->bundle_policy) {
		config.bundle_policy = (Peer::BundlePolicy)(raw->bundle_policy - 1);
	}

	if (raw->rtcp_mux_policy)
	{
		config.rtcp_mux_policy = (Peer::RtcpMuxPolicy)(raw->rtcp_mux_policy - 1);
	}

	if (raw->ice_servers)
	{
		config.servers = from_c(raw->ice_servers, raw->ice_servers_size);
	}

	return config;
}

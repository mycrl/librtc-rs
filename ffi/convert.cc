#include "ffi.h"
#include <string>
#include <vector>
#include <assert.h>
#include "api/peer_connection_interface.h"

const std::string from_c(char* raw)
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

	if (raw.credential)
	{
		server.password = from_c(raw.credential);
	}

	if (raw.username)
	{
		server.username = from_c(raw.username);
	}

	if (raw.urls)
	{
		server.urls = from_c(*raw.urls);
	}

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
	using Peer = webrtc::PeerConnectionInterface;

	Peer::RTCConfiguration config;
	config.enable_dtls_srtp = true;
	config.sdp_semantics = webrtc::SdpSemantics::kUnifiedPlan;
	config.ice_candidate_pool_size = raw->ice_candidate_pool_size;

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
		config.servers = from_c(*raw->ice_servers);
	}
	
	return config;
}

const webrtc::IceCandidateInterface* from_c(struct RTCIceCandidate* ice_candidate)
{
	int index = ice_candidate->sdp_mline_index;
	const std::string mid = from_c(ice_candidate->sdp_mid);
	const std::string candidate = from_c(ice_candidate->candidate);
	return webrtc::CreateIceCandidate(mid, index, candidate, nullptr);
}

const std::string sdp_type_to_string(enum RTC_SESSION_DESCRIPTION_TYPE type)
{
	if (type == RTC_SESSION_DESCRIPTION_TYPE_ANSWER) 
	{
		return "answer";
	} else
	if (type == RTC_SESSION_DESCRIPTION_TYPE_OFFER)
	{
		return "offer";
	} else
	if (type == RTC_SESSION_DESCRIPTION_TYPE_PRANSWER)
	{
		return "pranswer";
	}
	else
	{
		return "rollback";
	}
}

webrtc::SessionDescriptionInterface* from_c(struct RTCSessionDescription* desc)
{
	const std::string type = sdp_type_to_string(desc->type);
	const std::string sdp = from_c((char*)desc->sdp);
	return webrtc::CreateSessionDescription(type, sdp, nullptr);
}

struct RTCSessionDescription* into_c(webrtc::SessionDescriptionInterface* desc)
{
	auto c_desc = (struct RTCSessionDescription*)malloc(sizeof(struct RTCSessionDescription));
	if (!c_desc)
	{
		return NULL;
	}

	std::string sdp;
	desc->ToString(&sdp);
	c_desc->sdp = (char*)malloc(sizeof(char) * sdp.size());
	if (!c_desc->sdp)
	{
		return NULL;
	}

    strcpy((char*)c_desc->sdp, sdp.c_str());
	c_desc->type = (enum RTC_SESSION_DESCRIPTION_TYPE)(desc->GetType());

	return c_desc;
}
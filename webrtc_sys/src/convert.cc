#include "convert.h"
#include <string>
#include <vector>

const std::string from_c(char* raw)
{
	return std::string(raw);
}

std::vector<std::string> from_c(char** raw, int size)
{
	std::vector<std::string> strings;
	for (int i = 0; i < size; i++)
	{
		strings.push_back(from_c(raw[i]));
	}
	
	return strings;
}

webrtc::PeerConnectionInterface::IceServer from_c(RTCIceServer raw)
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
	config.enable_dtls_srtp = true;
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

const webrtc::IceCandidateInterface* from_c(RTCIceCandidate* ice_candidate)
{
	int index = ice_candidate->sdp_mline_index;
	const std::string mid = from_c(ice_candidate->sdp_mid);
	const std::string candidate = from_c(ice_candidate->candidate);
	return webrtc::CreateIceCandidate(mid, index, candidate, nullptr);
}

webrtc::SdpType from_c(RTCSessionDescriptionType type)
{
	if (type == RTCSessionDescriptionType::Answer)
	{
		return webrtc::SdpType::kAnswer;
	} 
	else
	if (type == RTCSessionDescriptionType::Offer)
	{
		return webrtc::SdpType::kOffer;
	} 
	else
	if (type == RTCSessionDescriptionType::PrAnswer)
	{
		return webrtc::SdpType::kPrAnswer;
	}
	else
	{
		return webrtc::SdpType::kRollback;
	}
}

std::unique_ptr<webrtc::SessionDescriptionInterface> from_c(
    RTCSessionDescription* desc)
{
	webrtc::SdpType type = from_c(desc->type);
	const std::string sdp = from_c((char*)desc->sdp);
	return webrtc::CreateSessionDescription(type, sdp);
}

RTCSessionDescription* into_c(webrtc::SessionDescriptionInterface* desc)
{
	auto c_desc = (RTCSessionDescription*)malloc(sizeof(RTCSessionDescription));
	if (!c_desc)
	{
        free(c_desc);
		return NULL;
	}

	std::string sdp;
	desc->ToString(&sdp);
	c_desc->sdp = (char*)malloc(sizeof(char) * (sdp.size() + 1));
	if (!c_desc->sdp)
	{
        free(c_desc);
		return NULL;
	}

    strcpy((char*)c_desc->sdp, sdp.c_str());
	c_desc->type = (RTCSessionDescriptionType)(desc->GetType());

	return c_desc;
}

void free_session_description(RTCSessionDescription* desc)
{
	free((void*)desc->sdp);
	free(desc);
}

ConnectionState into_c(webrtc::PeerConnectionInterface::PeerConnectionState state)
{
	using PeerConnectionState = webrtc::PeerConnectionInterface::PeerConnectionState;
	if (state == PeerConnectionState::kNew)
	{
		return ConnectionState::New;
	} 
	else
	if (state == PeerConnectionState::kConnecting)
	{
		return ConnectionState::Connecting;
	} 
	else
	if (state == PeerConnectionState::kConnected)
	{
		return ConnectionState::Connected;
	} 
	else
	if (state == PeerConnectionState::kDisconnected)
	{
		return ConnectionState::Disconnected;
	}
	else
	if (state == PeerConnectionState::kClosed)
	{
		return ConnectionState::Close;
	} 
	else
	{
		return ConnectionState::Failed;
	}
}

SignalingState into_c(webrtc::PeerConnectionInterface::SignalingState state)
{
	using kSignalingState = webrtc::PeerConnectionInterface::SignalingState;
	if (state == kSignalingState::kStable)
	{
		return SignalingState::Stable;
	}
	else
	if (state == kSignalingState::kHaveLocalOffer)
	{
		return SignalingState::HaveLocalOffer;
	}
	else
	if (state == kSignalingState::kHaveLocalPrAnswer)
	{
		return SignalingState::HaveLocalPrAnswer;
	}
	else
	if (state == kSignalingState::kHaveRemoteOffer)
	{
		return SignalingState::HaveRemoteOffer;
	}
	else
	if (state == kSignalingState::kHaveRemotePrAnswer)
	{
		return SignalingState::HaveRemotePrAnswer;
	}
	else
	{
		return SignalingState::Closed;
	}
}

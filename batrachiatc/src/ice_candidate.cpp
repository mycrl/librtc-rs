#include "ice_candidate.h"
#include "base.h"

/**
 * ice candidate
 */

const webrtc::IceCandidateInterface* from_c(RTCIceCandidate* ice_candidate)
{
	int index = ice_candidate->sdp_mline_index;
	const std::string mid = std::string(ice_candidate->sdp_mid);
	const std::string candidate = std::string(ice_candidate->candidate);
	return webrtc::CreateIceCandidate(mid, index, candidate, nullptr);
}

void free_ice_candidate(RTCIceCandidate* candidate)
{
	free_incomplete_ptr(candidate->candidate);
	free_incomplete_ptr(candidate->sdp_mid);
	free_incomplete_ptr(candidate);
}

RTCIceCandidate* into_c(webrtc::IceCandidateInterface* candidate)
{
	auto c_candidate = (RTCIceCandidate*)malloc(sizeof(RTCIceCandidate));
	if (!c_candidate)
	{
        free_ice_candidate(c_candidate);
		return NULL;
	}

	c_candidate->sdp_mid = (char*)malloc(sizeof(char) * (candidate->sdp_mid().size() + 1));
	if (!c_candidate->sdp_mid)
	{
        free_ice_candidate(c_candidate);
		return NULL;
	}

	std::string _candidate;
	candidate->ToString(&_candidate);
	c_candidate->candidate = (char*)malloc(sizeof(char) * (_candidate.size() + 1));
	if (!c_candidate->candidate)
	{
        free_ice_candidate(c_candidate);
		return NULL;
	}

	c_candidate->sdp_mline_index = candidate->sdp_mline_index();
	strcpy(c_candidate->sdp_mid, candidate->sdp_mid().c_str());
	strcpy(c_candidate->candidate, _candidate.c_str());
	return c_candidate;
}

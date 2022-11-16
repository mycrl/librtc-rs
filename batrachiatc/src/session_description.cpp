#include "session_description.h"

/**
 * offer / answer
 */

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
	const std::string sdp = std::string((char*)desc->sdp);
	return webrtc::CreateSessionDescription(type, sdp);
}

RTCSessionDescription* into_c(webrtc::SessionDescriptionInterface* desc)
{
	auto c_desc = (RTCSessionDescription*)malloc(sizeof(RTCSessionDescription));
	if (!c_desc)
	{
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

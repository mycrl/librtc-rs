#include "api/peer_connection_interface.h"
#include "observer.h"

void Observer::OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState new_state)
{
	if (this->_on_connectionstatechange_handler == NULL) return;
	this->_on_connectionstatechange_handler(new_state);
}

void Observer::OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel)
{
	if (this->_on_datachannel_handler == NULL) return;
	this->_on_datachannel_handler(data_channel);
}

void Observer::OnRenegotiationNeeded()
{
	if (this->_on_renegotiationneeded_handler == NULL) return;
	this->_on_renegotiationneeded_handler();
}

void Observer::OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState new_state)
{
	if (this->_on_icegatheringchange_handler == NULL) return;
	this->_on_icegatheringchange_handler(new_state);
}

void Observer::OnIceCandidate(const webrtc::IceCandidateInterface* candidate)
{
	if (this->_on_icecandidate_handler == NULL) return;
	this->_on_icecandidate_handler(candidate);
}

void Observer::set_connectionstatechange_handler(void (*handler)(webrtc::PeerConnectionInterface::SignalingState new_state))
{
	this->_on_connectionstatechange_handler = handler;
}

void Observer::set_datachannel_handler(void (*handler)(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel))
{
	this->_on_datachannel_handler = handler;
}

void Observer::set_renegotiationneeded_handler(void (*handler)())
{
	this->_on_renegotiationneeded_handler = handler;
}

void Observer::set_icegatheringchange_handler(void (*handler)(webrtc::PeerConnectionInterface::IceGatheringState new_state))
{
	this->_on_icegatheringchange_handler = handler;
}

void Observer::set_icecandidate_handler(void (*handler)(const webrtc::IceCandidateInterface* candidate))
{
	this->_on_icecandidate_handler = handler;
}

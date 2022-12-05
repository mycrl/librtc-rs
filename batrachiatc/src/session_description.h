#ifndef BATRACHIATC_SESSION_DESCRIPTION_H_
#define BATRACHIATC_SESSION_DESCRIPTION_H_
#pragma once

#include "api/peer_connection_interface.h"

/*
An enum describing the session description's type.
*/
typedef enum {
    /*
    The session description object describes the initial proposal in an 
    offer/answer exchange. The session negotiation process begins with an offer 
    being sent from the caller to the callee.
    */
    RTCSessionDescriptionTypeOffer,
    /*
    Description must be treated as an SDP answer, but not a final answer.
    */
    RTCSessionDescriptionTypePrAnswer,
    /*
    The SDP contained in the sdp property is the definitive choice in the 
    exchange. In other words, this session description describes the agreed-upon 
    configuration, and is being sent to finalize negotiation.
    */
    RTCSessionDescriptionTypeAnswer,
    /*
    This special type with an empty session description is used to
    roll back to the previous stable state.
    */
    RTCSessionDescriptionTypeRollback,
} RTCSessionDescriptionType;

/*
The RTCSessionDescription interface describes one end of a connection or 
potential connection and how it's configured. Each RTCSessionDescription 
consists of a description type indicating which part of the offer/answer 
negotiation process it describes and of the SDP descriptor of the session.
*/
typedef struct {
    RTCSessionDescriptionType type;
    /*
    A string containing the SDP describing the session.
    */
    const char* sdp;
} RTCSessionDescription;

webrtc::SdpType from_c(RTCSessionDescriptionType type);
std::unique_ptr<webrtc::SessionDescriptionInterface> from_c(RTCSessionDescription* desc);
RTCSessionDescription* into_c(webrtc::SessionDescriptionInterface* raw);
void free_session_description(RTCSessionDescription* raw);

#endif  // BATRACHIATC_SESSION_DESCRIPTION_H_
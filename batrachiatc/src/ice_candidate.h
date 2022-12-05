#ifndef BATRACHIATC_ICE_CANDIDATE_H_
#define BATRACHIATC_ICE_CANDIDATE_H_
#pragma once

#include "api/peer_connection_interface.h"

/*
RTCIceCandidate

The RTCIceCandidate interface¡ªpart of the WebRTC API¡ªrepresents a candidate 
Interactive Connectivity Establishment (ICE) configuration which may be used to 
establish an RTCPeerConnection.

An ICE candidate describes the protocols and routing needed for WebRTC to be 
able to communicate with a remote device. When starting a WebRTC peer connection, 
typically a number of candidates are proposed by each end of the connection, 
until they mutually agree upon one which describes the connection they decide 
will be best. WebRTC then uses that candidate's details to initiate the 
connection.

For details on how the ICE process works, see Lifetime of a WebRTC session.
The article WebRTC connectivity provides additional useful details.
*/
typedef struct {
    /*
    A string describing the properties of the candidate, taken directly from the 
    SDP attribute "candidate". The candidate string specifies the network 
    connectivity information for the candidate. If the candidate is an empty 
    string (""), the end of the candidate list has been reached; this candidate 
    is known as the "end-of-candidates" marker.
    */
    char* candidate;
    /*
    A string containing the identification tag of the media stream with which 
    the candidate is associated, or null if there is no associated media stream. 
    The default is null.
    */
    char* sdp_mid;
    /*
    A number property containing the zero-based index of the m-line with which 
    the candidate is associated, within the SDP of the media description, or 
    null if no such associated exists. The default is null.
    */
    int sdp_mline_index;
} RTCIceCandidate;

const webrtc::IceCandidateInterface* from_c(RTCIceCandidate* candidate);
RTCIceCandidate* into_c(webrtc::IceCandidateInterface* candidate);
void free_ice_candidate(RTCIceCandidate* candidate);

#endif  // BATRACHIATC_ICE_CANDIDATE_H_
#ifndef BATRACHIATC_PEER_CONNECTION_H_
#define BATRACHIATC_PEER_CONNECTION_H_
#pragma once

#include "api/peer_connection_interface.h"
#include "peer_connection_config.h"
#include "session_description.h"
#include "media_stream_track.h"
#include "ice_candidate.h"
#include "data_channel.h"
#include "observer.h"
#include "base.h"

/*
RTCPeerConnection

The RTCPeerConnection interface represents a WebRTC connection between the local 
computer and a remote peer. It provides methods to connect to a remote peer, 
maintain and monitor the connection, and close the connection once it's no 
longer needed.
*/
typedef struct {
    rtc::scoped_refptr<webrtc::PeerConnectionInterface> pc;
    rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> pc_factory;
} RTCPeerConnection;

/*
Returns a newly-created RTCPeerConnection, which represents a
connection between the local device and a remote peer.
*/
extern "C" EXPORT RTCPeerConnection* create_rtc_peer_connection(
    RTCPeerConnectionConfigure* config,
    Events* events,
    void* ctx);

/*
The RTCPeerConnection.close() method closes the current peer connection.

Calling this method terminates the RTCPeerConnection's ICE agent, ending any
ongoing ICE processing and any active streams. This also releases any resources
in use by the ICE agent, including TURN permissions. All RTCRtpSender objects
are considered to be stopped once this returns (they may still be in the process
of stopping, but for all intents and purposes, they're stopped).
*/
extern "C" EXPORT void rtc_close(RTCPeerConnection* peer);

extern "C" EXPORT void rtc_run();

/*
When a web site or app using RTCPeerConnection receives a new ICE candidate from 
the remote peer over its signaling channel, it delivers the newly-received 
candidate to the browser's ICE agent by calling RTCPeerConnection.addIceCandidate(). 
This adds this new remote candidate to the RTCPeerConnection's remote 
description, which describes the state of the remote end of the connection.

If the candidate parameter is missing or a value of null is given when calling 
addIceCandidate(), the added ICE candidate is an "end-of-candidates" indicator. 
The same is the case if the value of the specified object's candidate is either 
missing or an empty string (""), it signals that all remote candidates have been 
delivered.

The end-of-candidates notification is transmitted to the remote peer using a 
candidate with an a-line value of end-of-candidates.

During negotiation, your app will likely receive many candidates which you'll 
deliver to the ICE agent in this way, allowing it to build up a list of 
potential connection methods. This is covered in more detail in the articles 
WebRTC connectivity and Signaling and video calling.
*/
extern "C" EXPORT bool rtc_add_ice_candidate(RTCPeerConnection* peer,
    RTCIceCandidate* icecandidate);

/*
The createAnswer() method on the RTCPeerConnection interface creates an SDP 
answer to an offer received from a remote peer during the offer/answer 
negotiation of a WebRTC connection. The answer contains information about any 
media already attached to the session, codecs and options supported by the 
browser, and any ICE candidates already gathered. The answer is delivered to the 
returned Promise, and should then be sent to the source of the offer to continue 
the negotiation process.
*/
extern "C" EXPORT void rtc_create_answer(RTCPeerConnection* peer,
    CreateDescCallback callback,
    void* ctx);

/*
The createOffer() method of the RTCPeerConnection interface initiates the 
creation of an SDP offer for the purpose of starting a new WebRTC connection to 
a remote peer. The SDP offer includes information about any MediaStreamTrack 
objects already attached to the WebRTC session, codec, and options supported by 
the browser, and any candidates already gathered by the ICE agent, for the 
purpose of being sent over the signaling channel to a potential peer to request 
a connection or to update the configuration of an existing connection.
*/
extern "C" EXPORT void rtc_create_offer(RTCPeerConnection* peer,
    CreateDescCallback callback,
    void* ctx);

/*
The RTCPeerConnection method setLocalDescription() changes the local description 
associated with the connection. This description specifies the properties of the 
local end of the connection, including the media format. The method takes a 
single parameter¡ªthe session description¡ªand it returns a Promise which is 
fulfilled once the description has been changed, asynchronously.

If setLocalDescription() is called while a connection is already in place, it 
means renegotiation is underway (possibly to adapt to changing network conditions). 
Because descriptions will be exchanged until the two peers agree on a 
configuration, the description submitted by calling setLocalDescription() does 
not immediately take effect. Instead, the current connection configuration 
remains in place until negotiation is complete. Only then does the agreed-upon 
configuration take effect.
*/
extern "C" EXPORT void rtc_set_local_description(RTCPeerConnection* peer,
    RTCSessionDescription* desc,
    SetDescCallback callback,
    void* ctx);

/*
The RTCPeerConnection method setRemoteDescription() sets the specified session 
description as the remote peer's current offer or answer. The description 
specifies the properties of the remote end of the connection, including the 
media format. The method takes a single parameter¡ªthe session description and 
it returns a Promise which is fulfilled once the description has been changed, 
asynchronously.

This is typically called after receiving an offer or answer from another peer 
over the signaling server. Keep in mind that if setRemoteDescription() is called 
while a connection is already in place, it means renegotiation is underway 
(possibly to adapt to changing network conditions).

Because descriptions will be exchanged until the two peers agree on a 
configuration, the description submitted by calling setRemoteDescription() does 
not immediately take effect. Instead, the current connection configuration 
remains in place until negotiation is complete. Only then does the agreed-upon 
configuration take effect.
*/
extern "C" EXPORT void rtc_set_remote_description(RTCPeerConnection* peer,
    RTCSessionDescription* desc,
    SetDescCallback callback,
    void* ctx);

/*
The RTCPeerConnection method addTrack() adds a new media track to the set of
tracks which will be transmitted to the other peer.
*/
extern "C" EXPORT void rtc_add_track(RTCPeerConnection * rtc,
    MediaStreamTrack * track,
    char* stream_id);

extern "C" EXPORT RTCDataChannel* rtc_create_data_channel(RTCPeerConnection * rtc,
    char* label,
    DataChannelOptions * options);

#endif  // BATRACHIATC_PEER_CONNECTION_H_
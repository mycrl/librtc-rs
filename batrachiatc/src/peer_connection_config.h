#ifndef BATRACHIATC_PEER_CONNECTION_CONFIG_H_
#define BATRACHIATC_PEER_CONNECTION_CONFIG_H_
#pragma once

#include "api/peer_connection_interface.h"

/*
Specifies how to handle negotiation of candidates when the remote peer is not 
compatible with the SDP BUNDLE standard. If the remote endpoint is BUNDLE-aware,
all media tracks and data channels are bundled onto a single transport at the 
completion of negotiation, regardless of policy used, and any superfluous 
transports that were created initially are closed at that point.

In technical terms, a BUNDLE lets all media flow between two peers flow across 
a single 5-tuple; that is, from a single IP and port on one peer to a single IP 
and port on the other peer, using the same transport protocol.
*/
typedef enum {
    /*
    The ICE agent initially creates one RTCDtlsTransport for each type of 
    content added: audio, video, and data channels. If the remote endpoint is 
    not BUNDLE-aware, then each of these DTLS transports handles all the 
    communication for one type of data.
    */
    BundelPolicyBalanced = 1,
    /*
    The ICE agent initially creates one RTCDtlsTransport per media track and a 
    separate one for data channels. If the remote endpoint is not BUNDLE-aware, 
    everything is negotiated on these separate DTLS transports.
    */
    BundelPolicyMaxCompat,
    /*
    The ICE agent initially creates only a single RTCDtlsTransport to carry all 
    of the RTCPeerConnection's data. If the remote endpoint is not BUNDLE-aware, 
    then only a single track will be negotiated and the rest ignored.
    */
    BundelPolicyMaxBundle,
} BundelPolicy;

/*
The current ICE transport policy; if the policy isn't specified, all is assumed 
by default, allowing all candidates to be considered. Possible values are:
*/
typedef enum {
    IceTransportPolicyNone = 1,
    /*
    Only ICE candidates whose IP addresses are being relayed, such as those 
    being passed through a STUN or TURN server, will be considered.
    */
    IceTransportPolicyRelay,
    /*
    Only ICE candidates with public IP addresses will be considered.
    Removed from the specification's May 13, 2016 working draft.
    */
    IceTransportPolicyPublic,
    /*
    All ICE candidates will be considered.
    */
    IceTransportPolicyAll,
} IceTransportPolicy;

/*
The RTCP mux policy to use when gathering ICE candidates,
in order to support non-multiplexed RTCP.
Possible values are:
*/
typedef enum {
    /*
    Instructs the ICE agent to gather both RTP and RTCP candidates.
    If the remote peer can multiplex RTCP,
    then RTCP candidates are multiplexed atop the corresponding RTP candidates.
    Otherwise, both the RTP and RTCP candidates are returned, separately.
    */
    RtcpMuxPolicyNegotiate = 1,
    /*
    Tells the ICE agent to gather ICE candidates for only RTP,
    and to multiplex RTCP atop them. If the remote peer doesn't support RTCP 
    multiplexing, then session negotiation fails. This is the default value.
    */
    RtcpMuxPolicyRequire,
} RtcpMuxPolicy;

/*
RTCIceServer

An array of RTCIceServer objects, each describing one server which may be used 
by the ICE agent; these are typically STUN and/or TURN servers. 
If this isn't specified, the connection attempt will be made with no STUN or 
TURN server available, which limits the connection to local peers.
*/
typedef struct {
    /*
    The credential to use when logging into the server.
    This is only used if the RTCIceServer represents a TURN server.
    */
    char* credential;
    /*
    This required property is either a single string or an array of strings,
    each specifying a URL which can be used to connect to the server.
    */
    char** urls;
    int urls_size;
    int urls_capacity;
    /*
    If the RTCIceServer is a TURN server, then this is the username to use 
    during the authentication process.
    */
    char* username;
} RTCIceServer;

/*
RTCPeerConnection

The RTCPeerConnection is a newly-created RTCPeerConnection,
which represents a connection between the local device and a remote peer.
*/
typedef struct {
    BundelPolicy bundle_policy;
    IceTransportPolicy ice_transport_policy;
    /*
    TODO: 未实现
    A string which specifies the target peer identity for the RTCPeerConnection.
    If this value is set (it defaults to null), the RTCPeerConnection will not 
    connect to a remote peer unless it can successfully authenticate with the 
    given name.
    */
    char* peer_identity;
    RtcpMuxPolicy rtcp_mux_policy;
    RTCIceServer* ice_servers;
    int ice_servers_size;
    int ice_servers_capacity;
    /*
    An unsigned 16-bit integer value which specifies the size of the prefetched 
    ICE candidate pool.
    The default value is 0 (meaning no candidate prefetching will occur).
    You may find in some cases that connections can be established more quickly 
    by allowing the ICE agent to start fetching ICE candidates before you start 
    trying to connect, so that they're already available for inspection when 
    RTCPeerConnection.setLocalDescription() is called.
    */
    int ice_candidate_pool_size;
} RTCPeerConnectionConfigure;

std::vector<std::string> from_c(char** raw, int size);
webrtc::PeerConnectionInterface::IceServer from_c(RTCIceServer raw);
webrtc::PeerConnectionInterface::IceServers from_c(RTCIceServer* raw, int size);
webrtc::PeerConnectionInterface::RTCConfiguration from_c(RTCPeerConnectionConfigure* raw);

#endif  // BATRACHIATC_PEER_CONNECTION_CONFIG_H_
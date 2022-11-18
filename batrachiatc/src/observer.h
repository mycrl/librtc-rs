#pragma once

#include "api/peer_connection_interface.h"
#include "session_description.h"
#include "media_stream_track.h"
#include "ice_candidate.h"

typedef webrtc::PeerConnectionInterface RTC;
typedef rtc::scoped_refptr<webrtc::DataChannelInterface> DataChannel;

typedef enum {
    PeerConnectionStateNew,
    PeerConnectionStateConnecting,
    PeerConnectionStateConnected,
    PeerConnectionStateDisconnected,
    PeerConnectionStateClose,
    PeerConnectionStateFailed,
} PeerConnectionState;

typedef enum {
    SignalingStateStable,
    SignalingStateHaveLocalOffer,
    SignalingStateHaveLocalPrAnswer,
    SignalingStateHaveRemoteOffer,
    SignalingStateHaveRemotePrAnswer,
    SignalingStateClosed,
} SignalingState;

typedef enum {
    IceGatheringStateNew,
    IceGatheringStateGathering,
    IceGatheringStateComplete,
} IceGatheringState;

typedef enum {
    IceConnectionStateNew,
    IceConnectionStateChecking,
    IceConnectionStateConnected,
    IceConnectionStateCompleted,
    IceConnectionStateFailed,
    IceConnectionStateDisconnected,
    IceConnectionStateClosed,
    IceConnectionStateMax,
} IceConnectionState;

typedef struct
{
    void* ctx;

    void (*on_signaling_change)(void* ctx, SignalingState state);
    /*
    A datachannel event is sent to an RTCPeerConnection instance when an
    RTCDataChannel has been added to the connection, as a result of the remote peer
    calling RTCPeerConnection.createDataChannel().
    */
    void (*on_datachannel)(void* ctx, void* channel);

    void (*on_ice_gathering_change)(void* ctx, IceGatheringState state);

    void (*on_ice_candidate)(void* ctx, RTCIceCandidate* candidate);

    void (*on_renegotiation_needed)(void* ctx);
    
    void (*on_ice_connection_change)(void* ctx, IceConnectionState state);
    
    void (*on_track)(void* ctx, MediaStreamTrack* track);
    /*
    The connectionstatechange event is sent to the onconnectionstatechange event
    handler on an RTCPeerConnection object after a new track has been added to an
    RTCRtpReceiver which is part of the connection. The new connection state can be
    found in connectionState, and is one of the string values: new, connecting,
    connected, disconnected, failed, or closed.
    */
    void (*on_connection_change)(void* ctx, PeerConnectionState state);
} IObserver;

typedef void (*SetDescCallback)(const char* error, void* ctx);
typedef void (*CreateDescCallback)(const char* error, RTCSessionDescription* desc, void* ctx);

class Observer
    : public webrtc::PeerConnectionObserver
    , public rtc::RefCountInterface
{
public:
    Observer(IObserver* events);
    static Observer* Create(IObserver* events);
    void OnSignalingChange(RTC::SignalingState state);
    void OnDataChannel(DataChannel data_channel);
    void OnIceGatheringChange(RTC::IceGatheringState state);
    void OnIceCandidate(const webrtc::IceCandidateInterface* candidate);
    void OnRenegotiationNeeded();
    void OnIceConnectionChange(RTC::IceConnectionState state);
    void OnTrack(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver);
    void OnConnectionChange(RTC::PeerConnectionState state);
private:
    IObserver* _events;
};

class CreateDescObserver
    : public webrtc::CreateSessionDescriptionObserver
{
public:
    CreateDescObserver(CreateDescCallback callback, void* ctx);
    static CreateDescObserver* Create(CreateDescCallback callback, void* ctx);
    void OnSuccess(webrtc::SessionDescriptionInterface* desc);
    void OnFailure(webrtc::RTCError error);
private:
    CreateDescCallback _callback;
    void* _ctx;
};

class SetDescObserver
    : public webrtc::SetSessionDescriptionObserver 
{
public:
    SetDescObserver(SetDescCallback callback, void* ctx);
    static SetDescObserver* Create(SetDescCallback callback, void* ctx);
    virtual void OnSuccess();
    virtual void OnFailure(webrtc::RTCError error);
private:
    SetDescCallback _callback;
    void* _ctx;
};

PeerConnectionState into_c(RTC::PeerConnectionState state);
SignalingState into_c(RTC::SignalingState state);
IceGatheringState into_c(RTC::IceGatheringState state);
IceConnectionState into_c(RTC::IceConnectionState state);
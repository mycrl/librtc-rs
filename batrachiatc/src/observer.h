#pragma once

#include "api/peer_connection_interface.h"
#include "session_description.h"
#include "media_stream_track.h"
#include "ice_candidate.h"
#include "data_channel.h"

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
    void (*on_datachannel)(void* ctx, RTCDataChannel* channel);
    void (*on_ice_gathering_change)(void* ctx, IceGatheringState state);
    void (*on_ice_candidate)(void* ctx, RTCIceCandidate* candidate);
    void (*on_renegotiation_needed)(void* ctx);
    void (*on_ice_connection_change)(void* ctx, IceConnectionState state);
    void (*on_track)(void* ctx, MediaStreamTrack* track);
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
    void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState state);
    void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);
    void OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState state);
    void OnIceCandidate(const webrtc::IceCandidateInterface* candidate);
    void OnRenegotiationNeeded();
    void OnIceConnectionChange(webrtc::PeerConnectionInterface::IceConnectionState state);
    void OnTrack(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver);
    void OnConnectionChange(webrtc::PeerConnectionInterface::PeerConnectionState state);
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

PeerConnectionState into_c(webrtc::PeerConnectionInterface::PeerConnectionState state);
SignalingState into_c(webrtc::PeerConnectionInterface::SignalingState state);
IceGatheringState into_c(webrtc::PeerConnectionInterface::IceGatheringState state);
IceConnectionState into_c(webrtc::PeerConnectionInterface::IceConnectionState state);
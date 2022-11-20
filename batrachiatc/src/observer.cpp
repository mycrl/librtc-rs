#include "api/peer_connection_interface.h"
#include "observer.h"

/*
connection state
 */

PeerConnectionState into_c(RTC::PeerConnectionState state)
{
    if (state == RTC::PeerConnectionState::kNew)
    {
        return PeerConnectionStateNew;
    }
	else
    if (state == RTC::PeerConnectionState::kConnecting)
    {
        return PeerConnectionStateConnecting;
    }
    else
    if (state == RTC::PeerConnectionState::kConnected)
    {
        return PeerConnectionStateConnected;
    }
    else
    if (state == RTC::PeerConnectionState::kDisconnected)
    {
        return PeerConnectionStateDisconnected;
    }
    else
    if (state == RTC::PeerConnectionState::kClosed)
    {
        return PeerConnectionStateClose;
    }
    else
    {
        return PeerConnectionStateFailed;
    }
}

/*
signaling state
 */

SignalingState into_c(RTC::SignalingState state)
{
    if (state == RTC::SignalingState::kStable)
    {
        return SignalingStateStable;
    }
	else
    if (state == RTC::SignalingState::kHaveLocalOffer)
    {
        return SignalingStateHaveLocalOffer;
    }
    else
    if (state == RTC::SignalingState::kHaveLocalPrAnswer)
    {
        return SignalingStateHaveLocalPrAnswer;
    }
    else
    if (state == RTC::SignalingState::kHaveRemoteOffer)
    {
        return SignalingStateHaveRemoteOffer;
    }
    else
    if (state == RTC::SignalingState::kHaveRemotePrAnswer)
    {
        return SignalingStateHaveRemotePrAnswer;
    }
    else
    {
        return SignalingStateClosed;
    }
}

/*
ice gathering state
*/
IceGatheringState into_c(RTC::IceGatheringState state)
{
    if (state == RTC::IceGatheringState::kIceGatheringNew)
    {
        return IceGatheringStateNew;
    }
    else
    if (state == RTC::IceGatheringState::kIceGatheringGathering)
    {
        return IceGatheringStateGathering;
    }
    else
    {
        return IceGatheringStateComplete;
    }
}

/*
ice connection state
*/
IceConnectionState into_c(RTC::IceConnectionState state)
{
    if (state == RTC::IceConnectionState::kIceConnectionNew)
    {
        return IceConnectionStateNew;
    }
    else
    if (state == RTC::IceConnectionState::kIceConnectionChecking)
    {
        return IceConnectionStateChecking;
    }
    else
    if (state == RTC::IceConnectionState::kIceConnectionConnected)
    {
        return IceConnectionStateConnected;
    }
    else
    if (state == RTC::IceConnectionState::kIceConnectionCompleted)
    {
        return IceConnectionStateCompleted;
    }
    else
    if (state == RTC::IceConnectionState::kIceConnectionFailed)
    {
        return IceConnectionStateFailed;
    }
    else
    if (state == RTC::IceConnectionState::kIceConnectionDisconnected)
    {
        return IceConnectionStateDisconnected;
    }
    else
    if (state == RTC::IceConnectionState::kIceConnectionClosed)
    {
        return IceConnectionStateClosed;
    }
    else
    {
        return IceConnectionStateMax;
    }
}

Observer::Observer(IObserver* events)
{
    _events = events;
}

Observer* Observer::Create(IObserver* events)
{
    auto self = new rtc::RefCountedObject<Observer>(events);
    self->AddRef();
    return self;
}

void Observer::OnSignalingChange(RTC::SignalingState state)
{
    _events->on_signaling_change(_events->ctx, into_c(state));
}

void Observer::OnDataChannel(DataChannel data_channel)
{
    auto channel = create_data_channel(data_channel);
    _events->on_datachannel(_events->ctx, channel);
}

void Observer::OnRenegotiationNeeded()
{
    _events->on_renegotiation_needed(_events->ctx);
}

void Observer::OnIceGatheringChange(RTC::IceGatheringState state)
{
    _events->on_ice_gathering_change(_events->ctx, into_c(state));
}

void Observer::OnIceCandidate(const webrtc::IceCandidateInterface* candidate)
{
    auto ice_candidate = into_c((webrtc::IceCandidateInterface*)candidate);
    if (!ice_candidate)
    {
        return;
    }

    _events->on_ice_candidate(_events->ctx, ice_candidate);
    free_ice_candidate(ice_candidate);
}

void Observer::OnConnectionChange(RTC::PeerConnectionState state)
{
    _events->on_connection_change(_events->ctx, into_c(state));
}

void Observer::OnIceConnectionChange(RTC::IceConnectionState state)
{
    _events->on_ice_connection_change(_events->ctx, into_c(state));
}

void Observer::OnTrack(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver)
{
    webrtc::MediaStreamTrackInterface* track = transceiver->receiver()->track();
    if (track->kind() == webrtc::MediaStreamTrackInterface::kVideoKind)
    {
        auto sink = media_stream_video_track_from(static_cast<webrtc::VideoTrackInterface*>(track));
        _events->on_track(_events->ctx, sink);
    }
}

CreateDescObserver::CreateDescObserver(CreateDescCallback callback, void* ctx)
{
    _callback = callback;
    _ctx = ctx;
}

CreateDescObserver* CreateDescObserver::Create(CreateDescCallback callback, void* ctx)
{
    return new rtc::RefCountedObject<CreateDescObserver>(callback, ctx);
}

void CreateDescObserver::OnSuccess(webrtc::SessionDescriptionInterface* desc)
{
    auto res = into_c(desc);
    if (!res)
    {
        _callback("malloc failed", NULL, _ctx);
    } 
    else 
    {
        _callback(NULL, res, _ctx);
        free_session_description(res);
    }
}

void CreateDescObserver::OnFailure(webrtc::RTCError error)
{
    _callback(error.message(), NULL, _ctx);
}

SetDescObserver::SetDescObserver(SetDescCallback callback, void* ctx)
{
    _callback = callback;
    _ctx = ctx;
}

SetDescObserver* SetDescObserver::Create(SetDescCallback callback, void* ctx)
{
    return new rtc::RefCountedObject<SetDescObserver>(callback, ctx);
}

void SetDescObserver::OnSuccess()
{
    _callback(NULL, _ctx);
}

void SetDescObserver::OnFailure(webrtc::RTCError error)
{
    _callback(error.message(), _ctx);
}

#include "api/peer_connection_interface.h"
#include "observer.h"

/**
 * connection state
 */

ConnectionState into_c(RTC::PeerConnectionState state)
{
	using PeerConnectionState = RTC::PeerConnectionState;
	if (state == PeerConnectionState::kNew)
	{
		return ConnectionState::New;
	}
	else
    if (state == PeerConnectionState::kConnecting)
    {
        return ConnectionState::Connecting;
    }
    else
    if (state == PeerConnectionState::kConnected)
    {
        return ConnectionState::Connected;
    }
    else
    if (state == PeerConnectionState::kDisconnected)
    {
        return ConnectionState::Disconnected;
    }
    else
    if (state == PeerConnectionState::kClosed)
    {
        return ConnectionState::Close;
    }
    else
    {
        return ConnectionState::Failed;
    }
}

/**
 * signaling state
 */

SignalingState into_c(RTC::SignalingState state)
{
	using kSignalingState = RTC::SignalingState;
	if (state == kSignalingState::kStable)
	{
		return SignalingState::Stable;
	}
	else
    if (state == kSignalingState::kHaveLocalOffer)
    {
        return SignalingState::HaveLocalOffer;
    }
    else
    if (state == kSignalingState::kHaveLocalPrAnswer)
    {
        return SignalingState::HaveLocalPrAnswer;
    }
    else
    if (state == kSignalingState::kHaveRemoteOffer)
    {
        return SignalingState::HaveRemoteOffer;
    }
    else
    if (state == kSignalingState::kHaveRemotePrAnswer)
    {
        return SignalingState::HaveRemotePrAnswer;
    }
    else
    {
        return SignalingState::Closed;
    }
}

Observer::Observer(EventBus events)
{
    _events = events;
}

Observer* Observer::Create(EventBus events)
{
    auto self = new rtc::RefCountedObject<Observer>(events);
    self->AddRef();
    return self;
}

void Observer::OnSignalingChange(RTC::SignalingState state)
{
    _events.on_signalingchange(_events.ctx, into_c(state));
}

void Observer::OnDataChannel(DataChannel data_channel)
{
	
}

void Observer::OnRenegotiationNeeded()
{
	printf("OnRenegotiationNeeded\n");
}

void Observer::OnIceGatheringChange(RTC::IceGatheringState state)
{

}

void Observer::OnIceCandidate(const webrtc::IceCandidateInterface* candidate)
{
	
}

void Observer::OnConnectionChange(RTC::PeerConnectionState state)
{
    _events.on_connectionstatechange(_events.ctx, into_c(state));
}

void Observer::OnIceConnectionChange(
    RTC::IceConnectionState state)
{
}

void Observer::OnTrack(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver)
{

}

CreateDescObserver::CreateDescObserver(CreateDescCallback callback, void* ctx)
{
    _callback = callback;
    _ctx = ctx;
}

CreateDescObserver* CreateDescObserver::Create(CreateDescCallback callback, void* ctx)
{
    return new rtc::RefCountedObject<CreateDescObserver>(
        callback, ctx);
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
    return new rtc::RefCountedObject<SetDescObserver>(
        callback, ctx);
}

void SetDescObserver::OnSuccess()
{
    _callback(NULL, _ctx);
}

void SetDescObserver::OnFailure(webrtc::RTCError error)
{
    _callback(error.message(), _ctx);
}

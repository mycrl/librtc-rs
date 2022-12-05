#include "api/peer_connection_interface.h"
#include "observer.h"

/*
connection state
 */

PeerConnectionState into_c(webrtc::PeerConnectionInterface::PeerConnectionState state)
{
    if (state == webrtc::PeerConnectionInterface::PeerConnectionState::kNew)
    {
        return PeerConnectionStateNew;
    }
	else
    if (state == webrtc::PeerConnectionInterface::PeerConnectionState::kConnecting)
    {
        return PeerConnectionStateConnecting;
    }
    else
    if (state == webrtc::PeerConnectionInterface::PeerConnectionState::kConnected)
    {
        return PeerConnectionStateConnected;
    }
    else
    if (state == webrtc::PeerConnectionInterface::PeerConnectionState::kDisconnected)
    {
        return PeerConnectionStateDisconnected;
    }
    else
    if (state == webrtc::PeerConnectionInterface::PeerConnectionState::kClosed)
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

SignalingState into_c(webrtc::PeerConnectionInterface::SignalingState state)
{
    if (state == webrtc::PeerConnectionInterface::SignalingState::kStable)
    {
        return SignalingStateStable;
    }
	else
    if (state == webrtc::PeerConnectionInterface::SignalingState::kHaveLocalOffer)
    {
        return SignalingStateHaveLocalOffer;
    }
    else
    if (state == webrtc::PeerConnectionInterface::SignalingState::kHaveLocalPrAnswer)
    {
        return SignalingStateHaveLocalPrAnswer;
    }
    else
    if (state == webrtc::PeerConnectionInterface::SignalingState::kHaveRemoteOffer)
    {
        return SignalingStateHaveRemoteOffer;
    }
    else
    if (state == webrtc::PeerConnectionInterface::SignalingState::kHaveRemotePrAnswer)
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
IceGatheringState into_c(webrtc::PeerConnectionInterface::IceGatheringState state)
{
    if (state == webrtc::PeerConnectionInterface::IceGatheringState::kIceGatheringNew)
    {
        return IceGatheringStateNew;
    }
    else
    if (state == webrtc::PeerConnectionInterface::IceGatheringState::kIceGatheringGathering)
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
IceConnectionState into_c(webrtc::PeerConnectionInterface::IceConnectionState state)
{
    if (state == webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionNew)
    {
        return IceConnectionStateNew;
    }
    else
    if (state == webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionChecking)
    {
        return IceConnectionStateChecking;
    }
    else
    if (state == webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionConnected)
    {
        return IceConnectionStateConnected;
    }
    else
    if (state == webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionCompleted)
    {
        return IceConnectionStateCompleted;
    }
    else
    if (state == webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionFailed)
    {
        return IceConnectionStateFailed;
    }
    else
    if (state == webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionDisconnected)
    {
        return IceConnectionStateDisconnected;
    }
    else
    if (state == webrtc::PeerConnectionInterface::IceConnectionState::kIceConnectionClosed)
    {
        return IceConnectionStateClosed;
    }
    else
    {
        return IceConnectionStateMax;
    }
}

Observer::Observer(Events* events, void* ctx)
{
    _events = events;
    _ctx = ctx;
}

Observer* Observer::Create(Events* events, void* ctx)
{
    auto self = new rtc::RefCountedObject<Observer>(events, ctx);
    self->AddRef();
    return self;
}

void Observer::OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState state)
{
    if (!_ctx)
    {
        return;
    }

    _events->on_signaling_change(_ctx, into_c(state));
}

void Observer::OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel)
{
    if (!_ctx)
    {
        return;
    }

    auto channel = create_data_channel(data_channel);
    _events->on_datachannel(_ctx, channel);
}

void Observer::OnRenegotiationNeeded()
{
    if (!_ctx)
    {
        return;
    }

    _events->on_renegotiation_needed(_ctx);
}

void Observer::OnIceGatheringChange(webrtc::PeerConnectionInterface::IceGatheringState state)
{
    if (!_ctx)
    {
        return;
    }

    _events->on_ice_gathering_change(_ctx, into_c(state));
}

void Observer::OnIceCandidate(const webrtc::IceCandidateInterface* candidate)
{
    if (!_ctx)
    {
        return;
    }

    auto ice_candidate = into_c((webrtc::IceCandidateInterface*)candidate);
    if (!ice_candidate)
    {
        return;
    }

    _events->on_ice_candidate(_ctx, ice_candidate);
    free_ice_candidate(ice_candidate);
}

void Observer::OnConnectionChange(webrtc::PeerConnectionInterface::PeerConnectionState state)
{
    if (!_ctx)
    {
        return;
    }

    _events->on_connection_change(_ctx, into_c(state));
}

void Observer::OnIceConnectionChange(webrtc::PeerConnectionInterface::IceConnectionState state)
{
    if (!_ctx)
    {
        return;
    }

    _events->on_ice_connection_change(_ctx, into_c(state));
}

void Observer::OnTrack(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver)
{
    if (!_ctx)
    {
        return;
    }

    webrtc::MediaStreamTrackInterface* track = transceiver->receiver()->track().get();
    if (track->kind() == webrtc::MediaStreamTrackInterface::kVideoKind)
    {
        auto sink = media_stream_video_track_from(static_cast<webrtc::VideoTrackInterface*>(track));
        _events->on_track(_ctx, sink);
    }
    else
    if (track->kind() == webrtc::MediaStreamTrackInterface::kAudioKind) 
    {
        auto sink = media_stream_audio_track_from(static_cast<webrtc::AudioTrackInterface*>(track));
        _events->on_track(_ctx, sink);
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

#include "api/peer_connection_interface.h"
#include "convert.h"
#include "observer.h"

Observer::Observer(EventBus events)
{
    _events = events;
}

Observer* Create(EventBus events)
{
    auto self = new rtc::RefCountedObject<Observer>(events);
    self->AddRef();
    return self;
}

void Observer::OnSignalingChange(
    webrtc::PeerConnectionInterface::SignalingState state)
{
    _events.on_signalingchange(_events.ctx, into_c(state));
}

void Observer::OnDataChannel(
    rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel)
{
	
}

void Observer::OnRenegotiationNeeded()
{
	printf("OnRenegotiationNeeded\n");
}

void Observer::OnIceGatheringChange(
    webrtc::PeerConnectionInterface::IceGatheringState state)
{

}

void Observer::OnIceCandidate(const webrtc::IceCandidateInterface* candidate)
{
	
}

void Observer::OnConnectionChange(
    webrtc::PeerConnectionInterface::PeerConnectionState state)
{
    _events.on_connectionstatechange(_events.ctx, into_c(state));
}

void Observer::OnIceConnectionChange(
    webrtc::PeerConnectionInterface::IceConnectionState state)
{
}

void Observer::OnTrack(
    rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver)
{

}

CreateDescObserver::CreateDescObserver(CreateDescCallback callback, 
    void* ctx)
{
    _callback = callback;
    _ctx = ctx;
}

CreateDescObserver* CreateDescObserver::Create(
    CreateDescCallback callback, 
    void* ctx)
{
    return new rtc::RefCountedObject<CreateDescObserver>(
        callback, ctx);
}

void CreateDescObserver::OnSuccess(
    webrtc::SessionDescriptionInterface* desc)
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

SetDescObserver* SetDescObserver::Create(SetDescCallback callback, 
    void* ctx)
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

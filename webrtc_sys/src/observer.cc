#include "api/peer_connection_interface.h"
#include "convert.h"
#include "observer.h"

Observer::Observer(EventBus events)
{
    _events = events;
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

DummyCreateDescObserver::DummyCreateDescObserver(CreateDescCallback callback, 
    void* ctx)
{
    _callback = callback;
    _ctx = ctx;
}

DummyCreateDescObserver* DummyCreateDescObserver::Create(
    CreateDescCallback callback, 
    void* ctx)
{
    return new rtc::RefCountedObject<DummyCreateDescObserver>(
        callback, ctx);
}

void DummyCreateDescObserver::OnSuccess(
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

void DummyCreateDescObserver::OnFailure(const std::string& error)
{
    _callback(error.c_str(), NULL, _ctx);
}

DummySetDescObserver::DummySetDescObserver(SetDescCallback callback, void* ctx)
{
    _callback = callback;
    _ctx = ctx;
}

DummySetDescObserver* DummySetDescObserver::Create(SetDescCallback callback, 
    void* ctx)
{
    return new rtc::RefCountedObject<DummySetDescObserver>(
        callback, ctx);
}

void DummySetDescObserver::OnSuccess()
{
    _callback(NULL, _ctx);
}

void DummySetDescObserver::OnFailure(webrtc::RTCError error)
{
    _callback(error.message(), _ctx);
}

#include "api/peer_connection_interface.h"
#include "convert.h"
#include "observer.h"
#include "ffi.h"

void Observer::OnSignalingChange(
    webrtc::PeerConnectionInterface::SignalingState new_state)
{
	
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
    webrtc::PeerConnectionInterface::IceGatheringState new_state)
{

}

void Observer::OnIceCandidate(const webrtc::IceCandidateInterface* candidate)
{
	
}

void Observer::OnIceConnectionChange(
    webrtc::PeerConnectionInterface::IceConnectionState new_state)
{
    printf("OnIceConnectionChange\n");
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

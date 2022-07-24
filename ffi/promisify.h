#pragma once

#include "api/peer_connection_interface.h"

class CreateDescPromisify: public webrtc::CreateSessionDescriptionObserver 
{
public:
    CreateDescPromisify(void* ctx, void (*callback)(struct RTCSessionDescription* desc, void* ctx));
    void OnSuccess(webrtc::SessionDescriptionInterface* desc);
    void OnFailure(const std::string& error);
private:
    void (*_callback)(struct RTCSessionDescription* desc, void* ctx);
    void* _ctx;
};

class SetDescPromisify: public webrtc::SetSessionDescriptionObserver 
{
public:
    SetDescPromisify(void* ctx, void (*callback)(int res, void* ctx));
    void OnSuccess();
    void OnFailure(const std::string& error);
private:
    void (*_callback)(int res, void* ctx);
    void* _ctx;
};
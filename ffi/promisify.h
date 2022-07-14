#pragma once

#include "api/peer_connection_interface.h"

class CreateDescPromisify: public webrtc::CreateSessionDescriptionObserver {
public:
    CreateDescPromisify(void (*callback)(struct RTCSessionDescription* desc));
    void OnSuccess(webrtc::SessionDescriptionInterface* desc);
    void OnFailure(const std::string& error);
private:
    void (*_callback)(struct RTCSessionDescription* desc);
};

class SetDescPromisify: public webrtc::SetSessionDescriptionObserver {
public:
    SetDescPromisify(void (*callback)(int res));
    void OnSuccess();
    void OnFailure(const std::string& error);
private:
    void (*_callback)(int res);
};
#include "api/peer_connection_interface.h"
#include "promisify.h"
#include "convert.h"

CreateDescPromisify::CreateDescPromisify(void (*callback)(struct RTCSessionDescription* desc))
{
    this->_callback = callback;
}

void CreateDescPromisify::OnSuccess(webrtc::SessionDescriptionInterface* desc)
{
    auto c_desc = into_c(desc);
    if (this->_callback == NULL) return;
    this->_callback(&c_desc);
}

void CreateDescPromisify::OnFailure(const std::string& _error)
{
    if (this->_callback == NULL) return;
    this->_callback(nullptr);
}

SetDescPromisify::SetDescPromisify(void (*callback)(int))
{
    this->_callback = callback;
}

void SetDescPromisify::OnSuccess()
{
    if (this->_callback == NULL) return;
    this->_callback(1);
}

void SetDescPromisify::OnFailure(const std::string& _error)
{
    if (this->_callback == NULL) return;
    this->_callback(0);
}
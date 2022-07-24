#include "api/peer_connection_interface.h"
#include "promisify.h"
#include "convert.h"

CreateDescPromisify::CreateDescPromisify(
    void* ctx, 
    void (*callback)(struct RTCSessionDescription* desc, void* ctx)
)
{
    this->_callback = callback;
    this->_ctx = ctx;
}

void CreateDescPromisify::OnSuccess(webrtc::SessionDescriptionInterface* desc)
{
    printf("rtc_create_offer OnSuccess\n");
    if (this->_callback == NULL) return;
    this->_callback(into_c(desc), this->_ctx);
}

void CreateDescPromisify::OnFailure(const std::string& _error)
{
    printf("rtc_create_offer OnFailure\n");
    if (this->_callback == NULL) return;
    this->_callback(nullptr, this->_ctx);
}

SetDescPromisify::SetDescPromisify(
    void* ctx, 
    void (*callback)(int res, void* ctx)
)
{
    this->_callback = callback;
    this->_ctx = ctx;
}

void SetDescPromisify::OnSuccess()
{
    if (this->_callback == NULL) return;
    this->_callback(1, this->_ctx);
}

void SetDescPromisify::OnFailure(const std::string& _error)
{
    if (this->_callback == NULL) return;
    this->_callback(0, this->_ctx);
}
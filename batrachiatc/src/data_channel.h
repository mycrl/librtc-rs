#ifndef BATRACHIATC_DATA_CHANNEL_H_
#define BATRACHIATC_DATA_CHANNEL_H_
#pragma once

#include "api/peer_connection_interface.h"
#include "base.h"

typedef enum {
    DataStateConnecting,
    DataStateOpen,
    DataStateClosing,
    DataStateClosed
} DataState;

typedef enum {
    PriorityVeryLow = 1,
    PriorityLow,
    PriorityMedium,
    PriorityHigh,
} Priority;

typedef struct {
    // Deprecated. Reliability is assumed, and channel will be unreliable if
    // maxRetransmitTime or MaxRetransmits is set.
    bool reliable;
    // True if ordered delivery is required.
    bool ordered;
    // The max period of time in milliseconds in which retransmissions will be
    // sent. After this time, no more retransmissions will be sent.
    //
    // Cannot be set along with `maxRetransmits`.
    // This is called `maxPacketLifeTime` in the WebRTC JS API.
    // Negative values are ignored, and positive values are clamped to [0-65535]
    uint64_t max_retransmit_time;
    // The max number of retransmissions.
    //
    // Cannot be set along with `maxRetransmitTime`.
    // Negative values are ignored, and positive values are clamped to [0-65535]
    uint64_t max_retransmits;
    // This is set by the application and opaque to the WebRTC implementation.
    char* protocol;
    // True if the channel has been externally negotiated and we do not send an
    // in-band signalling in the form of an "open" message. If this is true, `id`
    // below must be set; otherwise it should be unset and will be negotiated
    // in-band.
    bool negotiated;
    // The stream id, or SID, for SCTP data channels. -1 if unset (see above).
    int id;
    Priority priority;
} DataChannelOptions;

class IDataChannel
    : public webrtc::DataChannelObserver
    , public rtc::RefCountInterface
{
public:
    IDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);
    static IDataChannel* From(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);
    void Send(uint8_t* buf, int size);
    void OnStateChange();
    void OnMessage(const webrtc::DataBuffer& buffer);
    void OnDataMessage(void* ctx, void(*handler)(void* _ctx, uint8_t* buf, uint64_t size));
    void RemoveOnMessage();

    DataState state;
private:
    rtc::scoped_refptr<webrtc::DataChannelInterface> _channel;
    void(*_handler)(void* ctx, uint8_t* buf, uint64_t size);
    void* _ctx;
};

/*
RTCDataChannel

The RTCDataChannel interface represents a network channel which can be used for
bidirectional peer-to-peer transfers of arbitrary data. Every data channel is
associated with an RTCPeerConnection, and each peer connection can have up to a
theoretical maximum of 65,534 data
channels (the actual limit may vary from browser to browser).
*/
typedef struct {
    char* label;
    IDataChannel* channel;
    bool remote;
} RTCDataChannel;

extern "C" EXPORT void data_channel_on_message(RTCDataChannel* channel,
    void(*handler)(void* ctx, uint8_t* buf, uint64_t size),
    void* ctx);

extern "C" EXPORT void data_channel_stop_on_message(
    RTCDataChannel* channel);

extern "C" EXPORT void data_channel_send(RTCDataChannel * channel, uint8_t * buf, int size);
extern "C" EXPORT DataState data_channel_get_state(RTCDataChannel * channel);
extern "C" EXPORT void free_data_channel(RTCDataChannel * channel);
    
webrtc::DataChannelInit* from_c(DataChannelOptions* options);
RTCDataChannel* create_data_channel(rtc::scoped_refptr<webrtc::DataChannelInterface> channel);

#endif  // BATRACHIATC_DATA_CHANNEL_H_
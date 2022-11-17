#pragma once

/*
RTCDataChannel

The RTCDataChannel interface represents a network channel which can be used for 
bidirectional peer-to-peer transfers of arbitrary data. Every data channel is 
associated with an RTCPeerConnection, and each peer connection can have up to a 
theoretical maximum of 65,534 data 
channels (the actual limit may vary from browser to browser).
*/
typedef struct {
    char* id;
    char* label;
} RTCDataChannel;

#include "src/sys.h"

void on_connectionstatechange(void* ctx, ConnectionState state)
{

}

void rtc_set_local_description_callback(const char* error, void* _ctx)
{
	printf("%s", error);
}

void rtc_create_offer_callback(const char* error, RTCSessionDescription* desc, void* ctx)
{
	RTCPeerConnection* pc = (RTCPeerConnection*)ctx;
	rtc_set_local_description(pc, desc, rtc_set_local_description_callback, NULL);
}

int main()
{
	EventBus events;
	events.on_connectionstatechange = on_connectionstatechange;

	RTCPeerConnection* pc = create_rtc_peer_connection(NULL, events);
	if (pc == NULL)
	{
		return -2;
	}

	rtc_create_offer(pc, rtc_create_offer_callback, pc);
	rtc_run();
	return 0;
}
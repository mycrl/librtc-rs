#pragma once

#ifdef WIN32
#define EXPORT __declspec(dllexport)
#endif

#ifdef WEBRTC_POSIX
#define EXPORT
#endif

void free_incomplete_ptr(void* ptr);

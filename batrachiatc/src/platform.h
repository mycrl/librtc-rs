#pragma once

#ifdef WIN32
#define EXPORT __declspec(dllexport)
#elif APPLE
#define EXPORT
#endif

void free_incomplete_ptr(void* ptr);

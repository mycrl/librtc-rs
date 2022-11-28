#pragma once

#define EXPORT __declspec(dllexport)

void free_incomplete_ptr(void* ptr);

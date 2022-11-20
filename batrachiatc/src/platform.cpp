#include "platform.h"
#include <cstdlib>

void free_incomplete_ptr(void* ptr)
{
	if (ptr)
	{
		free(ptr);
	}
}

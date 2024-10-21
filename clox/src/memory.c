#include "clox/memory.h"

#include <stddef.h>
#include <stdlib.h>

void *reallocate(void *ptr, size_t new_len)
{
	if (new_len == 0) {
		free(ptr);
		return NULL;
	}

	void *res = realloc(ptr, new_len);
	if (res == NULL) {
		exit(1);
	}

	return res;
}

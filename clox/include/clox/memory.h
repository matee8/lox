#ifndef CLOX_MEMORY_H
#define CLOX_MEMORY_H

#include <stddef.h>

#define GROW_CAP(cap) ((cap) < 8 ? 8 : (cap) * 2)
#define GROW_ARR(type, ptr, new_len) \
	(type *)reallocate(ptr, (new_len) * sizeof(type))
#define FREE_ARR(type, ptr) reallocate(ptr, 0)

void *reallocate(void *ptr, size_t new_len);

#endif

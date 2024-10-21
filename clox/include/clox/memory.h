#ifndef CLOX_MEMORY_H
#define CLOX_MEMORY_H

#include <stddef.h>

#define GROW_ARR(type, ptr, new_len) \
	(type *)reallocate(ptr, (new_len) * sizeof(type))

void *reallocate(void *ptr, size_t new_len);
size_t grow_capacity(size_t old_cap);
void free_array(void *ptr);

#endif

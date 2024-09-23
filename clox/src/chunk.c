#include "clox/chunk.h"

#include <stddef.h>
#include <stdint.h>

#include "clox/memory.h"

void chunk_init(chunk *c)
{
	c->len = 0;
	c->cap = 0;
	c->codes = NULL;
}

void chunk_write(chunk *c, uint8_t byte)
{
	if (c->cap < c->len + 1) {
		c->cap = GROW_CAP(c->cap);
		c->codes = GROW_ARR(uint8_t, c->codes, c->cap);
	}

	c->codes[c->len] = byte;
	c->len++;
}

void chunk_free(chunk *c)
{
	FREE_ARR(uint8_t, c->codes);
	chunk_init(c);
}

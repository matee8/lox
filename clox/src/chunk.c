#include "clox/chunk.h"

#include <stddef.h>
#include <stdint.h>

#include "clox/memory.h"
#include "clox/value.h"

void chunk_init(chunk *c)
{
	c->len = 0;
	c->cap = 0;
	c->codes = NULL;
	value_array_init(&c->constants);
}

void chunk_write(chunk *c, uint8_t byte)
{
	if (c->cap < c->len + 1) {
		c->cap = GROW_CAP(c->cap);
		c->codes = GROW_ARR(uint8_t, c->codes, c->cap);
	}

	c->codes[c->len] = byte;
	++c->len;
}

void chunk_free(chunk *c)
{
	FREE_ARR(uint8_t, c->codes);
	value_array_free(&c->constants);
	chunk_init(c);
}

size_t chunk_add_constant(chunk *c, value val)
{
	value_array_write(&c->constants, val);
	return c->constants.len - 1;
}

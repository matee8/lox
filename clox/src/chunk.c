#include "clox/chunk.h"

#include <stddef.h>
#include <stdint.h>

#include "clox/memory.h"
#include "clox/value.h"

void chunk_init(Chunk *c)
{
	c->len = 0;
	c->cap = 0;
	c->codes = NULL;
	c->lines = NULL;
	value_array_init(&c->constants);
}

void chunk_write(Chunk *c, uint8_t byte, int32_t line)
{
	if (c->cap < c->len + 1) {
		c->cap = GROW_CAP(c->cap);
		c->codes = GROW_ARR(uint8_t, c->codes, c->cap);
		c->lines = GROW_ARR(int32_t, c->lines, c->cap);
	}

	c->codes[c->len] = byte;
	c->lines[c->len] = line;
	++c->len;
}

void chunk_free(Chunk *c)
{
	(void)FREE_ARR(uint8_t, c->codes);
	(void)FREE_ARR(int32_t, c->lines);
	value_array_free(&c->constants);
	chunk_init(c);
}

size_t chunk_add_constant(Chunk *c, Value val)
{
	value_array_write(&c->constants, val);
	return c->constants.len - 1;
}

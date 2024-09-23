#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include "clox/chunk.h"
#include "clox/debug.h"

int main(void)
{
	chunk c;

	chunk_init(&c);
	size_t const_idx = chunk_add_constant(&c, 1.2);
	chunk_write(&c, OP_CONSTANT);
	chunk_write(&c, (uint8_t)const_idx);
	chunk_write(&c, OP_RETURN);
	__debug_disassemble_chunk(&c, "test chunk");
	chunk_free(&c);

	return EXIT_SUCCESS;
}

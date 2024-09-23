#include <stdlib.h>

#include "clox/chunk.h"
#include "clox/debug.h"

int main(void)
{
	chunk c;

	chunk_init(&c);
	chunk_write(&c, OP_RETURN);
	__debug_disassemble_chunk(&c, "test chunk");
	chunk_free(&c);

	return EXIT_SUCCESS;
}

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include "clox/chunk.h"
#include "clox/debug.h"
#include "clox/virtual_machine.h"

int main(void)
{
	chunk c;
	virtual_machine vm;

	virtual_machine_init(&vm);

	chunk_init(&c);

	size_t const_idx = chunk_add_constant(&c, 1.2);
	chunk_write(&c, OP_CONSTANT, 123);
	chunk_write(&c, (uint8_t)const_idx, 123);

	const_idx = chunk_add_constant(&c, 3.4);
	chunk_write(&c, OP_CONSTANT, 123);
	chunk_write(&c, const_idx, 123);

	chunk_write(&c, OP_ADD, 123);

	const_idx = chunk_add_constant(&c, 5.6);
	chunk_write(&c, OP_CONSTANT, 123);
	chunk_write(&c, const_idx, 123);

	chunk_write(&c, OP_DIVIDE, 123);
	chunk_write(&c, OP_NEGATE, 123);

	chunk_write(&c, OP_RETURN, 123);

#ifdef DEBUG_TRACE_EXECUTION
	__debug_disassemble_chunk(&c, "test chunk");
#endif

	virtual_machine_interpret(&vm, &c);

	chunk_free(&c);

	return EXIT_SUCCESS;
}

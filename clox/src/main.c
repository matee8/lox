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
	chunk_write(&c, OP_RETURN, 123);
	__debug_disassemble_chunk(&c, "test chunk");
    virtual_machine_interpret(&vm, &c);
	chunk_free(&c);

	return EXIT_SUCCESS;
}

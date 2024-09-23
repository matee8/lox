#include "clox/virtual_machine.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/chunk.h"
#include "clox/value.h"
#include "clox/debug.h"

// void virtual_machine_init(virtual_machine *vm)
// {
// }

interpret_result interpret(virtual_machine *vm, chunk *c)
{
	vm->chunk = c;
	vm->ip = &vm->chunk->codes[0];
	while (1) {
#define DEBUG_TRACE_EXECUTION
#ifdef DEBUG_TRACE_EXECUTION
		(void)__debug_disassemble_instruction(
			vm->chunk, (size_t)(vm->ip - vm->chunk->codes));
#endif
		uint8_t instruction = *vm->ip++;
		switch (instruction) {
		case OP_RETURN:
			return OK;
		case OP_CONSTANT: {
			size_t const_idx = *vm->ip++;
			value constant = vm->chunk->constants.values[const_idx];
			printf("%f\n", constant);
		}
		}
	}
}

// void virtual_machine_free(virtual_machine *vm)
// {
// }

#include "clox/virtual_machine.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/chunk.h"
#include "clox/value.h"
#include "clox/debug.h"

static void reset_stack(virtual_machine *vm)
{
    vm->stack_top = vm->stack;
}

void virtual_machine_init(virtual_machine *vm)
{
    reset_stack(vm);
}

interpret_result virtual_machine_interpret(virtual_machine *vm, chunk *c)
{
	vm->chunk = c;
	vm->ip = &vm->chunk->codes[0];
	while (1) {
#define DEBUG_TRACE_EXECUTION
#ifdef DEBUG_TRACE_EXECUTION
        printf("          ");
        for (value *slot = vm->stack; slot < vm->stack_top; slot++) {
            printf("[ %f ]", *slot);
        }
        printf("\n");
		(void)__debug_disassemble_instruction(
			vm->chunk, (size_t)(vm->ip - vm->chunk->codes));
#endif
		uint8_t instruction = *vm->ip++;
		switch (instruction) {
		case OP_RETURN:
            printf("%f\n", virtual_machine_pop(vm));
			return INTERPRET_OK;
		case OP_CONSTANT: {
			size_t const_idx = *vm->ip++;
			value constant = vm->chunk->constants.values[const_idx];
            virtual_machine_push(vm, constant);
			printf("%f\n", constant);
		}
		}
	}
}

void virtual_machine_push(virtual_machine *vm, value val)
{
    *vm->stack_top = val;
    vm->stack_top++;
}

value virtual_machine_pop(virtual_machine *vm)
{
    vm->stack_top--;
    return *vm->stack_top;
}

// void virtual_machine_free(virtual_machine *vm)
// {
// }

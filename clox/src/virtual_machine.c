#include "clox/virtual_machine.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/chunk.h"
#include "clox/compiler.h"
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

static interpret_result run(virtual_machine *vm, chunk *c)
{
	vm->chunk = c;
	vm->ip = &vm->chunk->codes[0];

#define READ_BYTE() (*vm->ip++)
#define BINARY_OP(op)                               \
	do {                                        \
		double b = virtual_machine_pop(vm); \
		double a = virtual_machine_pop(vm); \
		virtual_machine_push(vm, a op b);   \
	} while (0)

	while (1) {
#ifdef DEBUG_TRACE_EXECUTION
		printf("          ");
		for (value *slot = vm->stack; slot < vm->stack_top; slot++) {
			printf("[ %f ]", *slot);
		}
		printf("\n");
		(void)__debug_disassemble_instruction(
			vm->chunk, (size_t)(vm->ip - vm->chunk->codes));
#endif

		uint8_t instruction = READ_BYTE();
		switch (instruction) {
		case OP_CONSTANT: {
			value constant =
				vm->chunk->constants.values[READ_BYTE()];
			virtual_machine_push(vm, constant);
			break;
		}
		case OP_ADD:
			BINARY_OP(+);
			break;
		case OP_SUBTRACT:
			BINARY_OP(-);
			break;
		case OP_MULTIPLY:
			BINARY_OP(*);
			break;
		case OP_DIVIDE:
			BINARY_OP(/);
			break;
		case OP_NEGATE:
			virtual_machine_push(vm, -virtual_machine_pop(vm));
			break;
		case OP_RETURN:
			printf("%f\n", virtual_machine_pop(vm));
			return INTERPRET_OK;
		}
	}
#undef READ_BYTE
#undef BINARY_OP
}

interpret_result virtual_machine_interpret(virtual_machine *vm, const char *src)
{
    (void)vm;
    compile(src);
    return INTERPRET_OK;
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

#include "clox/virtual_machine.h"

#include <stddef.h>
#include <stdint.h>

#include "clox/chunk.h"
#include "clox/compiler.h"
#include "clox/value.h"

void virtual_machine_init(VirtualMachine *vm)
{
	vm->stack_top = vm->stack;
}

static InterpreterResult run(VirtualMachine *vm, Chunk *c)
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
#include "clox/debug.h"
		(void)puts("          ");
		for (Value *slot = vm->stack; slot < vm->stack_top; ++slot) {
			(void)printf("[ %f ]", *slot);
		}
		(void)puts("\n");
		(void)debug_disassemble_instruction(
			vm->chunk, (size_t)(vm->ip - vm->chunk->codes));
#endif

		OpCode instruction = 0;
		switch (instruction = READ_BYTE()) {
		case OP_CONSTANT: {
			Value constant =
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
			value_print(virtual_machine_pop(vm));
			return INTERPRET_OK;
		default:
			break;
		}
	}
#undef READ_BYTE
#undef BINARY_OP
}

InterpreterResult virtual_machine_interpret(VirtualMachine *vm, const char *src)
{
	Chunk chunk;
	chunk_init(&chunk);

	if (!compile(src, &chunk)) {
		chunk_free(&chunk);
		return INTERPRET_COMPILE_ERROR;
	}

	vm->chunk = &chunk;
	vm->ip = vm->chunk->codes;

	InterpreterResult result = run(vm, &chunk);

	chunk_free(&chunk);

	return result;
}

void virtual_machine_push(VirtualMachine *vm, Value val)
{
	*vm->stack_top = val;
	++vm->stack_top;
}

Value virtual_machine_pop(VirtualMachine *vm)
{
	vm->stack_top--;
	return *vm->stack_top;
}

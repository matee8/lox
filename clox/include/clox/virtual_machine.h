#ifndef CLOX_VIRTUAL_MACHINE_H
#define CLOX_VIRTUAL_MACHINE_H

#include "clox/value.h"
#include "clox/chunk.h"

#define STACK_MAX 256

typedef struct {
	chunk *chunk;
	uint8_t *ip;
	value *stack_top;
	value stack[STACK_MAX];
} virtual_machine;

typedef enum {
	INTERPRET_OK,
	INTERPRET_COMPILE_ERROR,
	INTERPRET_RUNTIME_ERROR
} interpret_result;

void virtual_machine_init(virtual_machine *vm);
interpret_result virtual_machine_interpret(virtual_machine *vm, chunk *c);
void virtual_machine_push(virtual_machine *vm, value val);
value virtual_machine_pop(virtual_machine *vm);
// void virtual_machine_free(virtual_machine *vm);

#endif

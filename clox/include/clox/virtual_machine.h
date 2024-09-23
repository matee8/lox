#ifndef CLOX_VIRTUAL_MACHINE_H
#define CLOX_VIRTUAL_MACHINE_H

#include "clox/chunk.h"

typedef struct {
	chunk *chunk;
	uint8_t *ip;
} virtual_machine;

typedef enum { OK, COMPILE_ERROR, RUNTIME_ERROR } interpret_result;

// void virtual_machine_init(virtual_machine *vm);
interpret_result interpret(virtual_machine *vm, chunk *c);
// void virtual_machine_free(virtual_machine *vm);

#endif

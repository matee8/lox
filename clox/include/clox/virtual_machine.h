#ifndef CLOX_VIRTUAL_MACHINE_H
#define CLOX_VIRTUAL_MACHINE_H

#include <stdint.h>

#include "clox/chunk.h"
#include "clox/value.h"

#define STACK_MAX 256

typedef struct __attribute__((aligned(128))) {
    Chunk *chunk;
    uint8_t *ip;
    Value *stack_top;
    Value stack[STACK_MAX];
} VirtualMachine;

typedef enum {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR
} InterpreterResult;

void virtual_machine_init(VirtualMachine *vm);
InterpreterResult virtual_machine_interpret(VirtualMachine *vm,
                                            const char *src);
void virtual_machine_push(VirtualMachine *vm, Value val);
Value virtual_machine_pop(VirtualMachine *vm);

#endif

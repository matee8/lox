#ifndef CLOX_CHUNK_H
#define CLOX_CHUNK_H

#include <stddef.h>
#include <stdint.h>

#include "clox/value.h"

typedef enum {
    OP_CONSTANT,
    OP_TRUE,
    OP_FALSE,
    OP_EQUAL,
    OP_GREATER,
    OP_LESS,
    OP_NIL,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
    OP_NOT,
    OP_NEGATE,
    OP_RETURN
} OpCode;

typedef struct {
    size_t len;
    size_t cap;
    uint8_t *codes;
    int32_t *lines;
    ValueArray constants;
} Chunk;

void chunk_init(Chunk *c);
void chunk_write(Chunk *c, uint8_t byte, int32_t line);
void chunk_free(Chunk *c);
size_t chunk_add_constant(Chunk *c, Value val);

#endif

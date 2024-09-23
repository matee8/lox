#ifndef CLOX_CHUNK_H
#define CLOX_CHUNK_H

#include <stddef.h>
#include <stdint.h>

#include "clox/value.h"

typedef enum {
    OP_CONSTANT,
    OP_RETURN
} opcode;

typedef struct {
    size_t len;
    size_t cap;
    uint8_t *codes;
    int32_t *lines;
    value_array constants;
} chunk;

void chunk_init(chunk *c);
void chunk_write(chunk *c, uint8_t byte, int32_t line);
void chunk_free(chunk *c);
size_t chunk_add_constant(chunk *c, value val);

#endif

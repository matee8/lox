#ifndef CLOX_CHUNK_H
#define CLOX_CHUNK_H

#include <stddef.h>
#include <stdint.h>

typedef enum {
    OP_RETURN
} opcode;

typedef struct {
    size_t len;
    size_t cap;
    uint8_t *codes;
} chunk;

void chunk_init(chunk *c);
void chunk_write(chunk *c, uint8_t byte);
void chunk_free(chunk *c);

#endif

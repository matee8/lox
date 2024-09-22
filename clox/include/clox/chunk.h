#ifndef CLOX_CHUNK_H
#define CLOX_CHUNK_H

#include <stddef.h>
#include <stdint.h>

enum opcode {
    OP_RETURN
};

struct chunk {
    size_t len;
    size_t cap;
    uint8_t *codes;
};

void chunk_init(struct chunk *c);
void chunk_write(struct chunk *c, uint8_t byte);
void chunk_free(struct chunk *c);

#endif

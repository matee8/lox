#ifndef CLOX_DEBUG_H
#define CLOX_DEBUG_H

#include <stddef.h>

#include "clox/chunk.h"

void debug_disassemble_chunk(const Chunk *c, const char *name);
size_t debug_disassemble_instruction(const Chunk *c, size_t offset);

#endif

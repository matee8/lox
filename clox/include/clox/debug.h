#ifndef CLOX_DEBUG_H
#define CLOX_DEBUG_H

#include <stddef.h>

#include "clox/chunk.h"

void __debug_disassemble_chunk(const chunk *c, const char *name);
size_t __debug_disassemble_instruction(const chunk *c, size_t offset);

#endif

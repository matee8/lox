#ifndef CLOX_DEBUG_H
#define CLOX_DEBUG_H

#include <stddef.h>

#include "clox/chunk.h"

void __debug_disassemble_chunk(const struct chunk *c, const char *name);

#endif

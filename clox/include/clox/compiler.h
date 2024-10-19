#ifndef CLOX_COMPILER_H
#define CLOX_COMPILER_H

#include <stdint.h>

#include "clox/chunk.h"

uint8_t compile(const char *src, Chunk *c);

#endif

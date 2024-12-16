#ifndef CLOX_COMPILER_H
#define CLOX_COMPILER_H

#include <stdbool.h>
#include <stdint.h>

#include "clox/chunk.h"

bool compile(const char *src, Chunk *c);

#endif

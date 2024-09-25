#ifndef CLOX_SCANNER_H
#define CLOX_SCANNER_H

#include <stdint.h>

typedef struct {
    const char *start;
    const char *current;
    int32_t line;
} scanner;

void scanner_init(scanner *sc, const char *src);

#endif

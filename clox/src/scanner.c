#include "clox/scanner.h"

void scanner_init(scanner *sc, const char *src)
{
    sc->start = src;
    sc->current = src;
    sc->line = 1;
}

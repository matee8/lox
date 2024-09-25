#include "clox/compiler.h"

#include "clox/scanner.h"

void compile(const char *src)
{
    scanner sc;
    scanner_init(&sc, src);
}

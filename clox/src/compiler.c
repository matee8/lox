#include "clox/compiler.h"

#include <stdint.h>

#include "clox/chunk.h"
#include "clox/scanner.h"

uint8_t compile(const char *src, chunk *c)
{
    (void)c;
	scanner sc;
	scanner_init(&sc, src);

    return 1;
}

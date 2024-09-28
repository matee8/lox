#include "clox/compiler.h"

#include <stdint.h>
#include <stdio.h>

#include "clox/scanner.h"

void compile(const char *src)
{
	scanner sc;
	scanner_init(&sc, src);

	int line = -1;
	while (1) {
		token t = scanner_scan_token(&sc);
		if (t.line != line) {
			(void)printf("%4d ", t.line);
			line = t.line;
		} else {
			(void)puts("   | ");
		}
		(void)printf("%2d '%.*s'\n", t.type, (int32_t)t.len, t.start);

		if (t.type == TOKEN_EOF)
			break;
	}
}

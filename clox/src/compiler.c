#include "clox/compiler.h"

#include <stdalign.h>
#include <stdint.h>
#include <stdio.h>

#include "clox/chunk.h"
#include "clox/scanner.h"

typedef struct __attribute__((aligned(128))) {
	token current;
	token previous;
	uint8_t had_error;
	uint8_t panic_mode;
} parser;

static void error_at(parser *p, token *t, const char *msg)
{
	if (p->panic_mode)
		return;
	p->panic_mode = 1;
	(void)fprintf(stderr, "[line %d] Error", t->line);

	if (t->type == TOKEN_EOF) {
		(void)fprintf(stderr, " at end");
	} else if (t->type != TOKEN_ERROR) {
		(void)fprintf(stderr, " at '%.*s'", (int32_t)t->len, t->start);
	}

	(void)fprintf(stderr, ": %s\n", msg);

	p->had_error = 1;
}

static void advance(parser *p, scanner *sc)
{
	p->previous = p->current;

	while (1) {
		p->current = scanner_scan_token(sc);
		if (p->current.type != TOKEN_ERROR)
			break;
		error_at(p, &p->current, p->current.start);
	}
}

static void consume(parser *p, scanner *sc, token_type type, const char *msg)
{
	if (p->current.type == type) {
		advance(p, sc);
		return;
	}

	error_at(p, &p->current, msg);
}

static void emit_byte(chunk *c, parser *p, uint8_t byte)
{
	chunk_write(c, byte, p->previous.line);
}

uint8_t compile(const char *src, chunk *c)
{
	scanner sc;
	parser p;

	scanner_init(&sc, src);

	p.had_error = 0;
	p.panic_mode = 0;

	advance(&p, &sc);
	consume(&p, &sc, TOKEN_EOF, "Expect end of expression.");
	emit_byte(c, &p, OP_RETURN);
	return !p.had_error;
}

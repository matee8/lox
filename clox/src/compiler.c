#include "clox/compiler.h"

#include <stdalign.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "clox/chunk.h"
#include "clox/scanner.h"

typedef struct __attribute__((aligned(128))) {
	Token current;
	Token previous;
	uint8_t had_error;
	uint8_t panic_mode;
} Parser;

typedef enum {
	PREC_NONE,
	PREC_ASSIGNMENT,
	PREC_OR,
	PREC_AND,
	PREC_EQUALITY,
	PREC_COMPARISON,
	PREC_TERM,
	PREC_FACTOR,
	PREC_UNARY,
	PREC_CALL,
	PREC_PRIMARY
} Precedence;

static void error_at(Parser *p, Token *t, const char *msg)
{
	if (p->panic_mode)
		return;
	p->panic_mode = 1;
	(void)fprintf(stderr, "[line %d] Error", t->line);

	if (t->type == TOKEN_EOF)
		(void)fprintf(stderr, " at end");
	else if (t->type != TOKEN_ERROR)
		(void)fprintf(stderr, " at '%.*s'", (int32_t)t->len, t->start);

	(void)fprintf(stderr, ": %s\n", msg);

	p->had_error = 1;
}

static void advance(Parser *p, Scanner *sc)
{
	p->previous = p->current;

	while (1) {
		p->current = scanner_scan_token(sc);
		if (p->current.type != TOKEN_ERROR)
			break;
		error_at(p, &p->current, p->current.start);
	}
}

static void consume(Parser *p, Scanner *sc, TokenType type, const char *msg)
{
	if (p->current.type == type) {
		advance(p, sc);
		return;
	}

	error_at(p, &p->current, msg);
}

static void parse_precedence(Precedence pr)
{
}

static void binary(Parser *p, Chunk *c)
{
	TokenType optype = p->previous.type;
	ParseRule *rule = get_rule(optype);
	parse_precedence((precedence)(rule->precedence + 1));

	switch (optype) {
	case TOKEN_PLUS:
		chunk_write(c, OP_ADD, p->previous.line);
		break;
	case TOKEN_MINUS:
		chunk_write(c, OP_SUBTRACT, p->previous.line);
		break;
	case TOKEN_STAR:
		chunk_write(c, OP_MULTIPLY, p->previous.line);
		break;
	case TOKEN_SLASH:
		chunk_write(c, OP_DIVIDE, p->previous.line);
		break;
	default:
		return;
	}
}

static void expression(void)
{
	parse_precedence(PREC_UNARY);
}

static void grouping(Parser *p, Scanner *sc)
{
	expression();
	consume(p, sc, TOKEN_RIGHT_PAREN, "Expect ')' after expression.");
}

static void number(Parser *p, Chunk *c)
{
	double value = strtod(p->previous.start, NULL);
	size_t const_idx = chunk_add_constant(c, value);
	if (const_idx > UINT8_MAX) {
		error_at(p, &p->current, "Too many constants in one chunk.");
		const_idx = 0;
	}
	chunk_write(c, OP_CONSTANT, p->previous.line);
	chunk_write(c, (uint8_t)const_idx, p->previous.line);
}

static void unary(Parser *p, Chunk *c)
{
	TokenType optype = p->previous.type;

	parse_precedence(PREC_UNARY);

	switch (optype) {
	case TOKEN_MINUS:
		chunk_write(c, OP_NEGATE, p->previous.line);
		break;
	default:
		return;
	}
}

uint8_t compile(const char *src, Chunk *c)
{
	Scanner sc;
	Parser p;

	scanner_init(&sc, src);

	p.had_error = 0;
	p.panic_mode = 0;

	advance(&p, &sc);
	expression();
	consume(&p, &sc, TOKEN_EOF, "Expect end of expression.");
	chunk_write(c, p.previous.line, OP_RETURN);
	return !p.had_error;
}

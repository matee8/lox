#include "clox/scanner.h"

#include <string.h>

void scanner_init(scanner *sc, const char *src)
{
	sc->start = src;
	sc->current = src;
	sc->line = 1;
}

static inline uint8_t is_digit(char c)
{
	return c >= '0' && c <= '9';
}

static inline uint8_t is_at_end(const scanner *sc)
{
	return *sc->current == '\0';
}

static inline char advance(scanner *sc)
{
	sc->current++;
	return sc->current[-1];
}

static inline char peek(const scanner *sc)
{
	return *sc->current;
}

static inline char peek_next(const scanner *sc)
{
	if (is_at_end(sc))
		return '\0';
	return sc->current[1];
}

static inline uint8_t match(scanner *sc, char expected)
{
	if (is_at_end(sc))
		return 0;
	if (*sc->current != expected)
		return 0;
	sc->current++;
	return 1;
}

static inline token make_token(const scanner *sc, token_type type)
{
	token t;
	t.type = type;
	t.start = sc->start;
	t.len = (size_t)(sc->current - sc->start);
	t.line = sc->line;
	return t;
}

static inline token error_token(const scanner *sc, const char *msg)
{
	token t;
	t.type = TOKEN_ERROR;
	t.start = msg;
	t.len = (size_t)strlen(msg);
	t.line = sc->line;
	return t;
}

static inline void skip_whitespace(scanner *sc)
{
	while (1) {
		char c = peek(sc);
		switch (c) {
		case ' ':
		case '\r':
		case '\t':
			advance(sc);
			break;
		case '\n':
			sc->line++;
			advance(sc);
			break;
		case '/':
			if (peek_next(sc) == '/')
				while (peek(sc) != '\n' && is_at_end(sc))
					advance(sc);
			else
				return;
			break;
		default:
			return;
		}
	}
}

static token string(scanner *sc)
{
	while (peek(sc) != '"' && !is_at_end(sc)) {
		if (peek(sc) == '\n')
			sc->line++;
		advance(sc);
	}

	if (is_at_end(sc))
		return error_token(sc, "Unterminated string.");

	advance(sc);
	return make_token(sc, TOKEN_STRING);
}

static token number(scanner *sc)
{
	while (is_digit(peek(sc)))
		advance(sc);

	if (peek(sc) == '.' && is_digit(peek_next(sc))) {
		advance(sc);

		while (is_digit(peek(sc)))
			advance(sc);
	}

	return make_token(sc, TOKEN_NUMBER);
}

token scan_token(scanner *sc)
{
	skip_whitespace(sc);
	sc->start = sc->current;

	if (is_at_end(sc))
		return make_token(sc, TOKEN_EOF);

	char c = advance(sc);

	switch (c) {
	case '(':
		return make_token(sc, TOKEN_LEFT_PAREN);
	case ')':
		return make_token(sc, TOKEN_RIGHT_PAREN);
	case '{':
		return make_token(sc, TOKEN_LEFT_BRACE);
	case '}':
		return make_token(sc, TOKEN_RIGHT_BRACE);
	case ';':
		return make_token(sc, TOKEN_SEMICOLON);
	case ',':
		return make_token(sc, TOKEN_COMMA);
	case '.':
		return make_token(sc, TOKEN_DOT);
	case '-':
		return make_token(sc, TOKEN_MINUS);
	case '+':
		return make_token(sc, TOKEN_PLUS);
	case '*':
		return make_token(sc, TOKEN_STAR);
	case '/':
		return make_token(sc, TOKEN_SLASH);
	case '!':
		return make_token(sc, match(sc, '=') ? TOKEN_BANG_EQUAL :
						       TOKEN_BANG);
	case '=':
		return make_token(sc, match(sc, '=') ? TOKEN_EQUAL_EQUAL :
						       TOKEN_EQUAL);
	case '<':
		return make_token(sc, match(sc, '=') ? TOKEN_LESS_EQUAL :
						       TOKEN_LESS);
	case '>':
		return make_token(sc, match(sc, '=') ? TOKEN_GREATER_EQUAL :
						       TOKEN_GREATER);
	case '"':
		return string(sc);
	case '0':
	case '1':
	case '2':
	case '3':
	case '4':
	case '5':
	case '6':
	case '7':
	case '8':
	case '9':
		return number(sc);
	}

	return error_token(sc, "Unexpected character.");
}

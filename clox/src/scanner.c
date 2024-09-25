#include "clox/scanner.h"

#include <string.h>

void scanner_init(scanner *sc, const char *src)
{
    sc->start = src;
    sc->current = src;
    sc->line = 1;
}

static uint8_t is_at_end(const scanner *sc) {
    return *sc->current == '\0';
}

static token make_token(const scanner *sc, token_type type) {
    token t;
    t.type = type;
    t.start = sc->start;
    t.len = (size_t)(sc->current - sc->start);
    t.line = sc->line;
    return t;
}

static token error_token(const scanner *sc, const char *msg) {
    token t;
    t.type = TOKEN_ERROR;
    t.start = msg;
    t.len = (size_t)strlen(msg);
    t.line = sc->line;
    return t;
}

token scan_token(scanner *sc)
{
    sc->start = sc->current;

    if (is_at_end(sc))
        return make_token(sc, TOKEN_EOF);

    return error_token(sc, "Unexpected character.");
}

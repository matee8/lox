#include "clox/compiler.h"

#include <stdalign.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "clox/chunk.h"
#include "clox/scanner.h"
#include "clox/value.h"

typedef struct {
    Token current;
    Token previous;
    bool had_error;
    bool panic_mode;
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

typedef void (*ParseFn)(Parser *p, Scanner *sc, Chunk *c);

typedef struct {
    ParseFn prefix;
    ParseFn infix;
    Precedence precedence;
} ParseRule;

static void unary(Parser *p, Scanner *sc, Chunk *c);
static void binary(Parser *p, Scanner *sc, Chunk *c);
static void grouping(Parser *p, Scanner *sc, Chunk *c);
static void number(Parser *p, Scanner *sc, Chunk *c);
static void literal(Parser *p, Scanner *sc, Chunk *c);

static const ParseRule rules[] = {
    [TOKEN_LEFT_PAREN] = {grouping, NULL, PREC_NONE},
    [TOKEN_RIGHT_PAREN] = {NULL, NULL, PREC_NONE},
    [TOKEN_LEFT_BRACE] = {NULL, NULL, PREC_NONE},
    [TOKEN_RIGHT_BRACE] = {NULL, NULL, PREC_NONE},
    [TOKEN_COMMA] = {NULL, NULL, PREC_NONE},
    [TOKEN_DOT] = {NULL, NULL, PREC_NONE},
    [TOKEN_MINUS] = {unary, binary, PREC_TERM},
    [TOKEN_PLUS] = {NULL, binary, PREC_TERM},
    [TOKEN_SEMICOLON] = {NULL, NULL, PREC_NONE},
    [TOKEN_SLASH] = {NULL, binary, PREC_FACTOR},
    [TOKEN_STAR] = {NULL, binary, PREC_FACTOR},
    [TOKEN_BANG] = {unary, NULL, PREC_EQUALITY},
    [TOKEN_BANG_EQUAL] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_EQUAL] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_EQUAL_EQUAL] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_GREATER] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_GREATER_EQUAL] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_LESS] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_LESS_EQUAL] = {NULL, binary, PREC_COMPARISON},
    [TOKEN_IDENTIFIER] = {NULL, NULL, PREC_NONE},
    [TOKEN_STRING] = {NULL, NULL, PREC_NONE},
    [TOKEN_NUMBER] = {number, NULL, PREC_NONE},
    [TOKEN_AND] = {NULL, NULL, PREC_NONE},
    [TOKEN_CLASS] = {NULL, NULL, PREC_NONE},
    [TOKEN_ELSE] = {NULL, NULL, PREC_NONE},
    [TOKEN_FALSE] = {literal, NULL, PREC_NONE},
    [TOKEN_FOR] = {NULL, NULL, PREC_NONE},
    [TOKEN_FUN] = {NULL, NULL, PREC_NONE},
    [TOKEN_IF] = {NULL, NULL, PREC_NONE},
    [TOKEN_NIL] = {literal, NULL, PREC_NONE},
    [TOKEN_OR] = {NULL, NULL, PREC_NONE},
    [TOKEN_PRINT] = {NULL, NULL, PREC_NONE},
    [TOKEN_RETURN] = {NULL, NULL, PREC_NONE},
    [TOKEN_SUPER] = {NULL, NULL, PREC_NONE},
    [TOKEN_THIS] = {NULL, NULL, PREC_NONE},
    [TOKEN_TRUE] = {literal, NULL, PREC_NONE},
    [TOKEN_VAR] = {NULL, NULL, PREC_NONE},
    [TOKEN_WHILE] = {NULL, NULL, PREC_NONE},
    [TOKEN_ERROR] = {NULL, NULL, PREC_NONE},
    [TOKEN_EOF] = {NULL, NULL, PREC_NONE},
};

static inline void error_at(Parser *p, const Token *t, const char *msg) {
    if (p->panic_mode) {
        return;
    }
    p->panic_mode = true;
    (void)fprintf(stderr, "[line %d] Error", t->line);

    if (t->type == TOKEN_EOF) {
        (void)fputs(" at end", stderr);
    } else if (t->type != TOKEN_ERROR) {
        (void)fprintf(stderr, " at '%.*s'", (int32_t)t->len, t->start);
    }

    (void)fprintf(stderr, ": %s\n", msg);

    p->had_error = true;
}

static void advance(Parser *p, Scanner *sc) {
    p->previous = p->current;

    for (;;) {
        p->current = scanner_scan_token(sc);
        if (p->current.type != TOKEN_ERROR) {
            break;
        }
        error_at(p, &p->current, p->current.start);
    }
}

static inline void consume(Parser *p, Scanner *sc, TokenType type,
                           const char *msg) {
    if (p->current.type == type) {
        advance(p, sc);
        return;
    }

    error_at(p, &p->current, msg);
}

static void parse_precedence(Parser *p, Scanner *sc, Chunk *c, Precedence pr) {
    advance(p, sc);
    const ParseFn prefix_rule = rules[p->previous.type].prefix;

    if (prefix_rule == NULL) {
        error_at(p, &p->current, "Expect expression.");
        return;
    }

    prefix_rule(p, sc, c);

    while (pr <= rules[p->current.type].precedence) {
        advance(p, sc);
        const ParseFn infix_rule = rules[p->previous.type].infix;
        infix_rule(p, sc, c);
    }
}

static inline void expression(Parser *p, Scanner *sc, Chunk *c) {
    parse_precedence(p, sc, c, PREC_ASSIGNMENT);
}

static void unary(Parser *p, Scanner *sc, Chunk *c) {
    (void)sc;
    const TokenType optype = p->previous.type;

    parse_precedence(p, sc, c, PREC_UNARY);

    switch (optype) {
        case TOKEN_BANG:
            chunk_write(c, OP_NOT, p->previous.line);
            break;
        case TOKEN_MINUS:
            chunk_write(c, OP_NEGATE, p->previous.line);
            break;
        default:
            return;
    }
}

static void binary(Parser *p, Scanner *sc, Chunk *c) {
    (void)sc;
    const TokenType optype = p->previous.type;
    const ParseRule *rule = &rules[optype];
    parse_precedence(p, sc, c, (Precedence)(rule->precedence + 1));

    switch (optype) {
        case TOKEN_BANG_EQUAL:
            chunk_write(c, OP_EQUAL, p->previous.line);
            chunk_write(c, OP_NOT, p->previous.line);
            break;
        case TOKEN_EQUAL_EQUAL:
            chunk_write(c, OP_EQUAL, p->previous.line);
            break;
        case TOKEN_GREATER:
            chunk_write(c, OP_GREATER, p->previous.line);
            break;
        case TOKEN_GREATER_EQUAL:
            chunk_write(c, OP_LESS, p->previous.line);
            chunk_write(c, OP_NOT, p->previous.line);
            break;
        case TOKEN_LESS:
            chunk_write(c, OP_LESS, p->previous.line);
            break;
        case TOKEN_LESS_EQUAL:
            chunk_write(c, OP_GREATER, p->previous.line);
            chunk_write(c, OP_NOT, p->previous.line);
            break;
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

static void grouping(Parser *p, Scanner *sc, Chunk *c) {
    (void)c;
    expression(p, sc, c);
    consume(p, sc, TOKEN_RIGHT_PAREN, "Expect ')' after expression.");
}

static void number(Parser *p, Scanner *sc, Chunk *c) {
    (void)sc;
    const double value = strtod(p->previous.start, NULL);
    size_t const_idx = chunk_add_constant(c, value_number(value));
    if (const_idx > UINT8_MAX) {
        error_at(p, &p->current, "Too many constants in one chunk.");
        const_idx = 0;
    }
    chunk_write(c, OP_CONSTANT, p->previous.line);
    chunk_write(c, (uint8_t)const_idx, p->previous.line);
}

static void literal(Parser *p, Scanner *sc, Chunk *c) {
    (void)sc;
    switch (p->previous.type) {
        case TOKEN_TRUE:
            chunk_write(c, OP_TRUE, p->previous.line);
            break;
        case TOKEN_FALSE:
            chunk_write(c, OP_FALSE, p->previous.line);
            break;
        case TOKEN_NIL:
            chunk_write(c, OP_NIL, p->previous.line);
            break;
        default:
            return;
    }
}

bool compile(const char *src, Chunk *c) {
    Scanner sc;
    Parser p;

    scanner_init(&sc, src);

    p.had_error = false;
    p.panic_mode = false;

    advance(&p, &sc);
    expression(&p, &sc, c);
    (void)fflush(stdout);
    consume(&p, &sc, TOKEN_EOF, "Expect end of expression.");
    chunk_write(c, OP_RETURN, p.previous.line);

#ifdef DEBUG_PRINT_CODE
#include "clox/debug.h"
    if (!p.had_error) {
        debug_disassemble_chunk(c, "code");
    }
#endif

    return !p.had_error;
}

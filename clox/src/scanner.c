#include "clox/scanner.h"

#include <stdint.h>
#include <string.h>

void scanner_init(Scanner *sc, const char *src) {
    sc->start = src;
    sc->current = src;
    sc->line = 1;
}

static inline uint8_t is_digit(char c) { return c >= '0' && c <= '9'; }

static inline uint8_t is_alpha(char c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

static inline uint8_t is_at_end(const Scanner *sc) {
    return *sc->current == '\0';
}

static inline char advance(Scanner *sc) {
    ++sc->current;
    return sc->current[-1];
}

static inline char peek(const Scanner *sc) { return *sc->current; }

static inline char peek_next(const Scanner *sc) {
    if (is_at_end(sc)) {
        return '\0';
    }
    return sc->current[1];
}

static inline uint8_t match(Scanner *sc, char expected) {
    if (is_at_end(sc)) {
        return 0;
    }
    if (*sc->current != expected) {
        return 0;
    }
    ++sc->current;
    return 1;
}

static inline Token make_token(const Scanner *sc, TokenType type) {
    const Token t = {.type = type,
                     .start = sc->start,
                     .len = (size_t)(sc->current - sc->start),
                     .line = sc->line};
    return t;
}

static inline Token error_token(const Scanner *sc, const char *msg) {
    const Token t = {
        .type = TOKEN_ERROR,
        .start = msg,
        .len = strlen(msg),
        .line = sc->line,
    };
    return t;
}

static inline void skip_whitespace(Scanner *sc) {
    for (;;) {
        const char c = peek(sc);
        switch (c) {
            case ' ':
            case '\r':
            case '\t':
                advance(sc);
                break;
            case '\n':
                ++sc->line;
                advance(sc);
                break;
            case '/':
                if (peek_next(sc) == '/') {
                    while (peek(sc) != '\n' && is_at_end(sc)) {
                        advance(sc);
                    }
                } else {
                    return;
                }
                break;
            default:
                return;
        }
    }
}

static inline TokenType check_keyword(const Scanner *sc, int32_t start,
                                      int32_t len, const char *rest,
                                      TokenType type) {
    if (sc->current - sc->start == start + len &&
        memcmp(sc->start + start, rest, len) == 0) {
        return type;
    }

    return TOKEN_IDENTIFIER;
}

static Token identifier(Scanner *sc) {
    while (is_alpha(peek(sc)) || is_digit(peek(sc))) {
        advance(sc);
    }

    TokenType type = TOKEN_IDENTIFIER;

    switch (sc->start[0]) {
        case 'a':
            type = check_keyword(sc, 1, 2, "nd", TOKEN_AND);
            break;
        case 'c':
            type = check_keyword(sc, 1, 4, "lass", TOKEN_CLASS);
            break;
        case 'e':
            type = check_keyword(sc, 1, 3, "lse", TOKEN_ELSE);
            break;
        case 'f':
            if (sc->current - sc->start > 1) {
                switch (sc->start[1]) {
                    case 'a':
                        type = check_keyword(sc, 2, 3, "lse", TOKEN_FALSE);
                        break;
                    case 'o':
                        type = check_keyword(sc, 2, 1, "r", TOKEN_FOR);
                        break;
                    case 'u':
                        type = check_keyword(sc, 2, 1, "n", TOKEN_FUN);
                        break;
                    default:
                        break;
                }
            }
            break;
        case 'i':
            type = check_keyword(sc, 1, 1, "f", TOKEN_IF);
            break;
        case 'n':
            type = check_keyword(sc, 1, 2, "il", TOKEN_NIL);
            break;
        case 'o':
            type = check_keyword(sc, 1, 1, "r", TOKEN_OR);
            break;
        case 'p':
            type = check_keyword(sc, 1, 4, "rint", TOKEN_PRINT);
            break;
        case 'r':
            type = check_keyword(sc, 1, 5, "eturn", TOKEN_RETURN);
            break;
        case 's':
            type = check_keyword(sc, 1, 4, "uper", TOKEN_SUPER);
            break;
        case 't':
            if (sc->current - sc->start > 1) {
                switch (sc->start[1]) {
                    case 'h':
                        type = check_keyword(sc, 2, 2, "is", TOKEN_THIS);
                        break;
                    case 'r':
                        type = check_keyword(sc, 2, 2, "ue", TOKEN_TRUE);
                        break;
                    default:
                        break;
                }
            }
            break;
        case 'v':
            type = check_keyword(sc, 1, 2, "ar", TOKEN_VAR);
            break;
        case 'w':
            type = check_keyword(sc, 1, 4, "hile", TOKEN_WHILE);
            break;
        default:
            type = TOKEN_IDENTIFIER;
            break;
    }

    return make_token(sc, type);
}

static Token string(Scanner *sc) {
    while (peek(sc) != '"' && !is_at_end(sc)) {
        if (peek(sc) == '\n') {
            ++sc->line;
        }
        advance(sc);
    }

    if (is_at_end(sc)) {
        return error_token(sc, "Unterminated string.");
    }

    advance(sc);
    return make_token(sc, TOKEN_STRING);
}

static Token number(Scanner *sc) {
    while (is_digit(peek(sc))) {
        advance(sc);
    }

    if (peek(sc) == '.' && is_digit(peek_next(sc))) {
        advance(sc);

        while (is_digit(peek(sc))) {
            advance(sc);
        }
    }

    return make_token(sc, TOKEN_NUMBER);
}

Token scanner_scan_token(Scanner *sc) {
    skip_whitespace(sc);
    sc->start = sc->current;

    if (is_at_end(sc)) {
        return make_token(sc, TOKEN_EOF);
    }

    const char c = advance(sc);

    if (is_alpha(c)) {
        return identifier(sc);
    }

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
            return make_token(sc,
                              match(sc, '=') ? TOKEN_BANG_EQUAL : TOKEN_BANG);
        case '=':
            return make_token(sc,
                              match(sc, '=') ? TOKEN_EQUAL_EQUAL : TOKEN_EQUAL);
        case '<':
            return make_token(sc,
                              match(sc, '=') ? TOKEN_LESS_EQUAL : TOKEN_LESS);
        case '>':
            return make_token(
                sc, match(sc, '=') ? TOKEN_GREATER_EQUAL : TOKEN_GREATER);
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
        default:
            return error_token(sc, "Unexpected character.");
    }
}

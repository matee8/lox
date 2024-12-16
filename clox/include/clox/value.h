#ifndef CLOX_VALUE_H
#define CLOX_VALUE_H

#include <stddef.h>
#include <stdint.h>

typedef enum { VAL_BOOL, VAL_NIL, VAL_NUMBER } ValueType;

typedef struct {
    ValueType type;
    union {
        uint8_t boolean;
        double number;
    } data;
} Value;

typedef struct {
    size_t len;
    size_t cap;
    Value *values;
} ValueArray;

void value_print(Value val);
void value_array_init(ValueArray *arr);
void value_array_write(ValueArray *arr, Value val);
void value_array_free(ValueArray *arr);

#endif

#ifndef CLOX_VALUE_H
#define CLOX_VALUE_H

#include <stddef.h>

typedef double Value;

typedef struct __attribute__((aligned(32))) {
    size_t len;
    size_t cap;
    Value *values;
} ValueArray;

void value_print(Value val);
void value_array_init(ValueArray *arr);
void value_array_write(ValueArray *arr, Value val);
void value_array_free(ValueArray *arr);

#endif

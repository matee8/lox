#ifndef CLOX_VALUE_H
#define CLOX_VALUE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef enum { VAL_BOOL, VAL_NIL, VAL_NUMBER } ValueType;

typedef struct {
    ValueType type;
    union {
        bool boolean;
        double number;
    } data;
} Value;

typedef struct {
    size_t len;
    size_t cap;
    Value *values;
} ValueArray;

Value value_nil(void);
Value value_bool(bool val);
Value value_number(double val);
bool value_as_bool(Value val);
double value_as_number(Value val);
bool value_is_nil(Value val);
bool value_is_bool(Value val);
bool value_is_number(Value val);
void value_print(Value val);
void value_array_init(ValueArray *arr);
void value_array_write(ValueArray *arr, Value val);
void value_array_free(ValueArray *arr);

#endif

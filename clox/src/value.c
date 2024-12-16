#include "clox/value.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/memory.h"

Value value_nil(void) {
    Value res = {.type = VAL_NIL, .data = NULL};
    return res;
}

Value value_bool(bool val) {
    Value res = {.type = VAL_BOOL, .data = {.boolean = val}};
    return res;
}

Value value_number(double val) {
    Value res = {.type = VAL_NUMBER, .data = {.number = val}};
    return res;
}

bool value_as_bool(Value val) {
    return val.data.boolean;
}

double value_as_double(Value val) {
    return val.data.number;
}

bool value_is_nil(Value val) {
    return val.type == VAL_NIL;
}

bool value_is_bool(Value val) {
    return val.type == VAL_BOOL;
}

bool value_is_number(Value val) {
    return val.type == VAL_NUMBER;
}

void value_print(Value val) { (void)printf("%g", val); }

void value_array_init(ValueArray *arr) {
    arr->len = 0;
    arr->cap = 0;
    arr->values = NULL;
}

void value_array_write(ValueArray *arr, Value val) {
    if (arr->cap < arr->len + 1) {
        arr->cap = grow_capacity(arr->cap);
        arr->values = GROW_ARR(Value, arr->values, arr->cap);
    }

    arr->values[arr->len] = val;
    ++arr->len;
}

void value_array_free(ValueArray *arr) {
    free_array(arr->values);
    value_array_init(arr);
}

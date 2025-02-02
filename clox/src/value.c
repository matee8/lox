#include "clox/value.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/memory.h"

Value value_nil(void) {
    Value res = {.type = VAL_NIL, .data = {0}};
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

bool value_as_bool(Value val) { return val.data.boolean; }

double value_as_number(Value val) { return val.data.number; }

bool value_is_nil(Value val) { return val.type == VAL_NIL; }

bool value_is_bool(Value val) { return val.type == VAL_BOOL; }

bool value_is_number(Value val) { return val.type == VAL_NUMBER; }

void value_print(Value val) {
    switch (val.type) {
        case VAL_BOOL:
            (void)printf(value_as_bool(val) ? "true" : "false");
            break;
        case VAL_NIL:
            (void)printf("nil");
            break;
        case VAL_NUMBER:
            (void)printf("%g", value_as_number(val));
            break;
    }
}

bool value_equals(Value lhs, Value rhs) {
    if (lhs.type != rhs.type) {
        return false;
    }
    switch (lhs.type) {
        case VAL_BOOL:
            return value_as_bool(lhs) == value_as_bool(rhs);
        case VAL_NIL:
            return true;
        case VAL_NUMBER:
            return value_as_number(lhs) == value_as_number(rhs);
        default:
            return false;
    }
}

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

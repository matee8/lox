#ifndef CLOX_VALUE_H
#define CLOX_VALUE_H

#include <stddef.h>

typedef double value;

typedef struct {
	size_t len;
	size_t cap;
	value *values;
} value_array;

void value_print(value val);
void value_array_init(value_array *arr);
void value_array_write(value_array *arr, value val);
void value_array_free(value_array *arr);

#endif

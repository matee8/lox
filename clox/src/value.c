#include "clox/value.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/memory.h"

void value_print(Value val)
{
	(void)printf("%g", val);
}

void value_array_init(ValueArray *arr)
{
	arr->len = 0;
	arr->cap = 0;
	arr->values = NULL;
}

void value_array_write(ValueArray *arr, Value val)
{
	if (arr->cap < arr->len + 1) {
		arr->cap = grow_capacity(arr->cap);
		arr->values = GROW_ARR(Value, arr->values, arr->cap);
	}

	arr->values[arr->len] = val;
	++arr->len;
}

void value_array_free(ValueArray *arr)
{
	free_array(arr->values);
	value_array_init(arr);
}

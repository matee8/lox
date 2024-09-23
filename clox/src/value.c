#include "clox/value.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/memory.h"

void value_print(value val)
{
	printf("%g", val);
}

void value_array_init(value_array *arr)
{
	arr->len = 0;
	arr->cap = 0;
	arr->values = NULL;
}

void value_array_write(value_array *arr, value val)
{
	if (arr->cap < arr->len + 1) {
		arr->cap = GROW_CAP(arr->cap);
		arr->values = GROW_ARR(value, arr->values, arr->cap);
	}

	arr->values[arr->len] = val;
	++arr->len;
}

void value_array_free(value_array *arr)
{
	FREE_ARR(value, arr->values);
	value_array_init(arr);
	;
}

#include "clox/debug.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/value.h"

static inline size_t constant_instruction(const char *name, const chunk *c,
					  size_t offset)
{
	size_t const_idx = (size_t)c->codes[offset + 1];
	printf("%-16s %4lu '", name, const_idx);
	value_print(c->constants.values[const_idx]);
	printf("'\n");
	return offset + 2;
}

static inline size_t simple_instruction(const char *name, size_t offset)
{
	printf("%s\n", name);
	return offset + 1;
}

static size_t disassemble_instruction(const chunk *c, size_t offset)
{
	printf("%04lu ", offset);

	if (offset > 0 && c->lines[offset] == c->lines[offset - 1])
		printf("   | ");
	else
		printf("%4d ", c->lines[offset]);

	opcode instruction = c->codes[offset];
	switch (instruction) {
	case OP_CONSTANT:
		return constant_instruction("OP_CONSTANT", c, offset);
	case OP_RETURN:
		return simple_instruction("OP_RETURN", offset);
	default:
		printf("Unknown opcode %d\n", instruction);
		return offset + 1;
	}
}

void __debug_disassemble_chunk(const chunk *c, const char *name)
{
	printf("== %s ==\n", name);

	size_t offset = 0;

	while (offset < c->len)
		offset = disassemble_instruction(c, offset);
}

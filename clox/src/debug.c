#include "clox/debug.h"

#include <stddef.h>
#include <stdio.h>

static inline size_t simple_instruction(const char *name, size_t offset)
{
	printf("%s\n", name);
	return offset + 1;
}

static size_t disassemble_instruction(const chunk *c, size_t offset)
{
	printf("%04lu ", offset);

	opcode instruction = c->codes[offset];
	switch (instruction) {
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

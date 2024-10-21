#include "clox/debug.h"

#include <stddef.h>
#include <stdio.h>

#include "clox/chunk.h"
#include "clox/value.h"

static inline size_t constant_instruction(const char *name, const Chunk *c,
					  size_t offset)
{
	size_t const_idx = (size_t)c->codes[offset + 1];
	(void)printf("%-16s %4lu '", name, const_idx);
	value_print(c->constants.values[const_idx]);
	(void)puts("'\n");
	return offset + 2;
}

static inline size_t simple_instruction(const char *name, size_t offset)
{
	(void)printf("%s\n", name);
	return offset + 1;
}

size_t debug_disassemble_instruction(const Chunk *c, size_t offset)
{
	(void)printf("%04lu ", offset);

	if (offset > 0 && c->lines[offset] == c->lines[offset - 1])
		(void)puts("   | ");
	else
		(void)printf("%4d ", c->lines[offset]);

	OpCode instruction = c->codes[offset];
	switch (instruction) {
	case OP_CONSTANT:
		return constant_instruction("OP_CONSTANT", c, offset);
	case OP_ADD:
		return simple_instruction("OP_ADD", offset);
	case OP_SUBTRACT:
		return simple_instruction("OP_SUBTRACT", offset);
	case OP_MULTIPLY:
		return simple_instruction("OP_MULTIPLY", offset);
	case OP_DIVIDE:
		return simple_instruction("OP_DIVIDE", offset);
	case OP_NEGATE:
		return simple_instruction("OP_NEGATE", offset);
	case OP_RETURN:
		return simple_instruction("OP_RETURN", offset);
	default:
		(void)printf("Unknown opcode %d\n", instruction);
		return offset + 1;
	}
}

void debug_disassemble_chunk(const Chunk *c, const char *name)
{
	(void)printf("== %s ==\n", name);

	size_t offset = 0;

	while (offset < c->len)
		offset = debug_disassemble_instruction(c, offset);
}

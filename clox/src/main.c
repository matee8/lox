#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "clox/virtual_machine.h"

static void repl(virtual_machine *vm)
{
	char line[1024];

	while (1) {
		printf("> ");

		if (!fgets(line, sizeof(line), stdin)) {
			printf("\n");
			break;
		}
	}

	virtual_machine_interpret(vm, line);
}

static char *read_file(const char *path)
{
	FILE *file = fopen(path, "rb");
	if (file == NULL) {
		fprintf(stderr, "Couldn't open file \"%s\".\n", path);
		exit(74);
	}

	fseek(file, 0L, SEEK_END);
	size_t file_size = ftell(file);
	rewind(file);

	char *buff = (char *)malloc(file_size + 1);
	if (buff == NULL) {
		fprintf(stderr, "Not enough memory to read \"%s\".\n", path);
		exit(74);
	}

	size_t bytes_read = fread(buff, sizeof(char), file_size, file);
	if (bytes_read < file_size) {
		fprintf(stderr, "Couldn't read file \"%s\".\n", path);
		exit(74);
	}
	buff[bytes_read] = '\0';

	fclose(file);
	return buff;
}

static void run_file(virtual_machine *vm, const char *path)
{
	char *src = read_file(path);
	interpret_result res = virtual_machine_interpret(vm, src);
	free(src);
	if (res == INTERPRET_COMPILE_ERROR)
		exit(65);
	else if (res == INTERPRET_RUNTIME_ERROR)
		exit(70);
}

int main(int argc, const char *argv[])
{
	virtual_machine vm;

	virtual_machine_init(&vm);

	if (argc == 1) {
		repl(&vm);
	} else if (argc == 2) {
		run_file(&vm, argv[1]);
	} else {
		fprintf(stderr, "Usage: clox [path]\n");
		exit(64);
	}

	return EXIT_SUCCESS;
}

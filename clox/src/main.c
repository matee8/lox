#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sysexits.h>

#include "clox/virtual_machine.h"

#define REPL_MAX_LINES 1024

static void repl(virtual_machine *vm)
{
	char line[REPL_MAX_LINES];

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
	FILE *file = fopen(path, "rbe");
	if (file == NULL) {
		(void)fprintf(stderr, "Couldn't open file \"%s\".\n", path);
		exit(EX_IOERR);
	}

	(void)fseek(file, 0L, SEEK_END);
	size_t file_size = ftell(file);
	(void)fseek(file, 0L, SEEK_SET);

	char *buff = (char *)malloc(file_size + 1);
	if (buff == NULL) {
		(void)fprintf(stderr, "Not enough memory to read \"%s\".\n",
			      path);
		exit(EX_IOERR);
	}

	size_t bytes_read = fread(buff, sizeof(char), file_size, file);
	if (bytes_read < file_size) {
		(void)fprintf(stderr, "Couldn't read file \"%s\".\n", path);
		exit(EX_IOERR);
	}
	buff[bytes_read] = '\0';

	(void)fclose(file);
	return buff;
}

static void run_file(virtual_machine *vm, const char *path)
{
	char *src = read_file(path);
	interpret_result res = virtual_machine_interpret(vm, src);
	free(src);
	if (res == INTERPRET_COMPILE_ERROR)
		exit(EX_DATAERR);
	else if (res == INTERPRET_RUNTIME_ERROR)
		exit(EX_SOFTWARE);
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
		(void)fputs("Usage: clox [path]\n", stderr);
		exit(EX_USAGE);
	}

	return EXIT_SUCCESS;
}

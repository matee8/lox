#include "clox/virtual_machine.h"

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#include "clox/chunk.h"
#include "clox/compiler.h"
#include "clox/value.h"

void virtual_machine_init(VirtualMachine *vm) { vm->stack_top = vm->stack; }

static inline uint8_t read_byte(VirtualMachine *vm) { return *vm->ip++; }

static inline Value peek(VirtualMachine *vm, uint8_t distance) {
    return vm->stack_top[-1 - distance];
}

static inline bool is_falsey(Value val) {
    return value_is_nil(val) || (value_is_bool(val) && !value_as_bool(val));
}

static inline void runtime_error(VirtualMachine *vm, const char *format, ...) {
    va_list args;
    va_start(args, format);
    (void)vfprintf(stderr, format, args);
    va_end(args);
    (void)fputs("\n", stderr);

    size_t instruction = vm->ip - vm->chunk->codes - 1;
    int32_t line = vm->chunk->lines[instruction];
    (void)fprintf(stderr, "[line %d] in script\n", line);
}

static inline InterpreterResult handle_binary_op(VirtualMachine *vm,
                                                 Value (*op)(double, double)) {
    if (!value_is_number(peek(vm, 0)) || !value_is_number(peek(vm, 1))) {
        runtime_error(vm, "Operands must be numbers.");
        return INTERPRET_RUNTIME_ERROR;
    }
    const double b = value_as_number(virtual_machine_pop(vm));
    const double a = value_as_number(virtual_machine_pop(vm));
    virtual_machine_push(vm, op(a, b));
    return INTERPRET_OK;
}

static inline Value add_op(double a, double b) { return value_number(a + b); }
static inline Value subtract_op(double a, double b) {
    return value_number(a - b);
}
static inline Value multiply_op(double a, double b) {
    return value_number(a * b);
}
static inline Value divide_op(double a, double b) {
    return value_number(a / b);
}
static inline Value greater_op(double a, double b) { return value_bool(a > b); }
static inline Value less_op(double a, double b) { return value_bool(a < b); }

static InterpreterResult run(VirtualMachine *vm, Chunk *c) {
    vm->chunk = c;
    vm->ip = &vm->chunk->codes[0];

    while (true) {
#ifdef DEBUG_TRACE_EXECUTION
#include "clox/debug.h"
        (void)puts("          ");
        for (Value *slot = vm->stack; slot < vm->stack_top; ++slot) {
            (void)printf("[ %f ]", *slot);
        }
        (void)puts("\n");
        (void)debug_disassemble_instruction(
            vm->chunk, (size_t)(vm->ip - vm->chunk->codes));
#endif

        switch (read_byte(vm)) {
            case OP_CONSTANT: {
                const Value constant =
                    vm->chunk->constants.values[read_byte(vm)];
                virtual_machine_push(vm, constant);
                break;
            }
            case OP_TRUE:
                virtual_machine_push(vm, value_bool(true));
                break;
            case OP_FALSE:
                virtual_machine_push(vm, value_bool(false));
                break;
            case OP_EQUAL: {
                Value a = virtual_machine_pop(vm);
                Value b = virtual_machine_pop(vm);
                virtual_machine_push(vm, value_bool(value_equals(a, b)));
                break;
            }
            case OP_GREATER:
                handle_binary_op(vm, greater_op);
                break;
            case OP_LESS:
                handle_binary_op(vm, less_op);
                break;
            case OP_NIL:
                virtual_machine_push(vm, value_nil());
                break;
            case OP_ADD:
                if (handle_binary_op(vm, add_op) != INTERPRET_OK) {
                    return INTERPRET_RUNTIME_ERROR;
                }
                break;
            case OP_SUBTRACT:
                if (handle_binary_op(vm, subtract_op) != INTERPRET_OK) {
                    return INTERPRET_RUNTIME_ERROR;
                }
                break;
            case OP_MULTIPLY:
                if (handle_binary_op(vm, multiply_op) != INTERPRET_OK) {
                    return INTERPRET_RUNTIME_ERROR;
                }
                break;
            case OP_DIVIDE:
                if (handle_binary_op(vm, divide_op) != INTERPRET_OK) {
                    return INTERPRET_RUNTIME_ERROR;
                }
                break;
            case OP_NOT:
                virtual_machine_push(
                    vm, value_bool(is_falsey(virtual_machine_pop(vm))));
                break;
            case OP_NEGATE:
                if (value_is_number(peek(vm, 0))) {
                    runtime_error(vm, "Operand must be a number.");
                    return INTERPRET_RUNTIME_ERROR;
                }
                virtual_machine_push(
                    vm,
                    value_number(-value_as_number(virtual_machine_pop(vm))));
                break;
            case OP_RETURN:
                value_print(virtual_machine_pop(vm));
                (void)fputs("\n", stdout);
                return INTERPRET_OK;
            default:
                break;
        }
    }

#undef READ_BYTE
}

InterpreterResult virtual_machine_interpret(VirtualMachine *vm,
                                            const char *src) {
    Chunk chunk;
    chunk_init(&chunk);

    if (!compile(src, &chunk)) {
        chunk_free(&chunk);
        return INTERPRET_COMPILE_ERROR;
    }

    vm->chunk = &chunk;
    vm->ip = vm->chunk->codes;

    const InterpreterResult result = run(vm, &chunk);

    chunk_free(&chunk);

    return result;
}

void virtual_machine_push(VirtualMachine *vm, Value val) {
    *vm->stack_top = val;
    ++vm->stack_top;
}

Value virtual_machine_pop(VirtualMachine *vm) {
    vm->stack_top--;
    return *vm->stack_top;
}

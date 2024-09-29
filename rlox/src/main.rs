use core::cmp::Ordering;
use std::{env, process};

use rlox::vm::{InterpretError, RunFileError, Vm};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm = Vm::new();

    match args.len().cmp(&1) {
        Ordering::Less => {
            eprintln!("Usage: rlox [path]");
            process::exit(exitcode::USAGE);
        }
        Ordering::Equal => {
            if vm.repl().is_err() {
                eprintln!("Failed to read from stdin or write to stdout.");
                process::exit(exitcode::IOERR);
            }
        }
        Ordering::Greater => match vm.run_file(&args[1]) {
            Ok(()) => {}
            Err(RunFileError::Io) => {
                eprintln!("Failed to open or read file {}.", args[1]);
                process::exit(exitcode::IOERR);
            }
            Err(RunFileError::Interpret(InterpretError::Compile)) => {
                eprintln!("Failed to compile file {}.", args[1]);
                process::exit(exitcode::DATAERR);
            }
            Err(RunFileError::Interpret(InterpretError::Runtime)) => {
                eprintln!(
                    "Runtime error occured while running file {}.",
                    args[1]
                );
                process::exit(exitcode::SOFTWARE);
            }
            Err(_) => {
                eprintln!(
                    "Unexpected error happened while running file {}.",
                    args[1]
                );
                process::exit(exitcode::UNAVAILABLE);
            }
        },
    }
}

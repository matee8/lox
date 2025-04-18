use core::cmp::Ordering;
use std::{env, process};

use rlox::vm::Vm;

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
        Ordering::Greater => {
            #[expect(
                clippy::indexing_slicing,
                reason = "`Ordering::Greater` ensures `args.len()` >= 2"
            )]
            let file_name = &args[1];
            if vm.run_file(file_name).is_err() {
                eprintln!("Failed to open or read file {file_name}.");
                process::exit(exitcode::IOERR);
            }
        }
    }
}

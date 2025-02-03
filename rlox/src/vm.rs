use core::ops::{Add, Div, Mul, Sub};
use std::{
    collections::VecDeque,
    fs,
    io::{self, Write as _},
    path::Path,
};

use thiserror::Error;

use crate::{
    chunk::{Chunk, OpCode},
    compiler,
    value::Value,
};

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum InterpretError {
    #[error("Compile time error.")]
    Compile,
    #[error("Runtime error.")]
    Runtime,
}

#[derive(Debug, Default)]
pub struct Vm {
    stack: VecDeque<Value>,
    chunk: Option<Chunk>,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum RunFileError {
    #[error("Failed to open or read file.")]
    Io,
    #[error(transparent)]
    Interpret(#[from] InterpretError),
}

impl Vm {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            stack: VecDeque::with_capacity(256),
            chunk: None,
        }
    }

    #[inline]
    pub fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;

            if line.is_empty() {
                break Ok(());
            }

            #[expect(
                clippy::let_underscore_untyped,
                clippy::let_underscore_must_use,
                reason = r#"
                    REPL intentionally ignores errors to maintain interactive
                    session.
                "#
            )]
            let _ = self.interpret(line);
        }
    }

    #[inline]
    pub fn run_file<P>(&mut self, path: P) -> Result<(), RunFileError>
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(path).or(Err(RunFileError::Io))?;

        self.interpret(contents)?;

        Ok(())
    }

    #[inline]
    fn interpret<S>(&mut self, source: S) -> Result<(), InterpretError>
    where
        S: AsRef<str>,
    {
        let mut chunk = Chunk::new();

        if compiler::compile(source.as_ref(), &mut chunk).is_err() {
            return Err(InterpretError::Compile);
        }

        self.chunk = Some(chunk);

        self.run()
    }

    fn run(&mut self) -> Result<(), InterpretError> {
        if let Some(chunk) = self.chunk.take() {
            for code in chunk.codes {
                match code {
                    OpCode::Constant(const_idx) => {
                        let constant = chunk
                            .constants
                            .get(const_idx)
                            .ok_or_else(|| {
                                self.runtime_error(
                                    "Stack underflow in const opcode.",
                                );
                                InterpretError::Runtime
                            })?;
                        self.stack.push_back(*constant);
                    }
                    OpCode::Add => {
                        if self.binary_op(Add::add).is_err() {
                            eprintln!("Stack underflow in add opcode.");
                            return Err(InterpretError::Compile);
                        }
                    }
                    OpCode::Subtract => {
                        if self.binary_op(Sub::sub).is_err() {
                            eprintln!("Stack underflow in sub opcode.");
                            return Err(InterpretError::Compile);
                        }
                    }
                    OpCode::Multiply => {
                        if self.binary_op(Mul::mul).is_err() {
                            eprintln!("Stack underflow in mul opcode.");
                            return Err(InterpretError::Compile);
                        }
                    }
                    OpCode::Divide => {
                        if self.binary_op(Div::div).is_err() {
                            eprintln!("Stack underflow in div opcode.");
                            return Err(InterpretError::Compile);
                        }
                    }
                    OpCode::Negate => {
                        let value = self.stack.pop_back().ok_or_else(|| {
                            self.runtime_error(
                                "Stack underflow in neg opcode.",
                            );
                            InterpretError::Runtime
                        })?;
                        let number = value.as_number().ok_or_else(|| {
                            self.runtime_error("Operand must be a number.");
                            InterpretError::Runtime
                        })?;
                        self.stack.push_back(Value::Number(-number));
                    }
                    OpCode::Return => {
                        if let Some(val) = self.stack.pop_back() {
                            println!("{val:?}");
                            return Ok(());
                        }
                        eprintln!("Stack underflow.");
                        return Err(InterpretError::Compile);
                    }
                }
            }
        }
        Ok(())
    }

    fn binary_op<T>(&mut self, op: T) -> Result<(), InterpretError>
    where
        T: FnOnce(f64, f64) -> f64,
    {
        let b = self
            .stack
            .pop_back()
            .ok_or_else(|| {
                eprintln!("Stack is empty.");
                InterpretError::Runtime
            })?
            .as_number()
            .ok_or_else(|| {
                self.runtime_error("Operands must be numbers");
                InterpretError::Runtime
            })?;
        let a = self
            .stack
            .pop_back()
            .ok_or_else(|| {
                self.runtime_error("Stack is empty.");
                InterpretError::Runtime
            })?
            .as_number()
            .ok_or_else(|| {
                self.runtime_error("Operands must be numbers.");
                InterpretError::Runtime
            })?;

        self.stack.push_back(Value::Number(op(a, b)));

        Ok(())
    }

    fn runtime_error(&self, msg: &str) {
        eprintln!("{msg}");
        let line = self.chunk.as_ref().map_or(&0, |chunk| chunk.lines.first().unwrap_or(&0));
        eprintln!("[line {line}] in script");
    }
}

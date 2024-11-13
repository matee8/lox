use std::{
    collections::VecDeque,
    fs,
    io::{self, Write},
    path::Path,
};

use thiserror::Error;

use crate::{chunk::Chunk, compiler, value::Value};

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum InterpretError {
    #[error("Compile time error.")]
    Compile,
    #[error("Runtime error.")]
    Runtime,
}

#[derive(Debug, Error)]
#[error("Stack is empty.")]
struct StackIsEmptyError;

#[derive(Debug, Default)]
pub struct Vm {
    stack: VecDeque<Value>,
    chunk: Option<Chunk>,
    ip: Option<usize>,
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
            ip: None,
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
                reason = "the repl doesn't care about errors."
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

        if compiler::compile(source, &mut chunk).is_err() {
            return Err(InterpretError::Compile);
        }

        self.chunk = Some(chunk);
        self.ip = Some(0);

        Ok(())
    }

    // #[inline]
    // pub fn interpret(&mut self) -> Result<(), InterpretError> {
    //     for code in self.chunk.codes() {
    //         match *code {
    //             OpCode::Constant(const_idx) => {
    //                 let Some(constant) = self.chunk.constants().get(const_idx)
    //                 else {
    //                     eprintln!("Stack underflow.");
    //                     return Err(InterpretError::Compile);
    //                 };
    //                 self.stack.push_back(*constant);
    //             }
    //             OpCode::Add => {
    //                 if self.binary_op(Add::add).is_err() {
    //                     eprintln!("Stack underflow.");
    //                     return Err(InterpretError::Compile);
    //                 }
    //             }
    //             OpCode::Subtract => {
    //                 if self.binary_op(Sub::sub).is_err() {
    //                     eprintln!("Stack underflow");
    //                     return Err(InterpretError::Compile);
    //                 }
    //             }
    //             OpCode::Multiply => {
    //                 if self.binary_op(Mul::mul).is_err() {
    //                     eprintln!("Stack underflow");
    //                     return Err(InterpretError::Compile);
    //                 }
    //             }
    //             OpCode::Divide => {
    //                 if self.binary_op(Div::div).is_err() {
    //                     eprintln!("Stack underflow");
    //                     return Err(InterpretError::Compile);
    //                 }
    //             }
    //             OpCode::Negate => {
    //                 if let Some(val) = self.stack.pop_back() {
    //                     self.stack.push_back(-val);
    //                 } else {
    //                     eprintln!("Stack underflow.");
    //                     return Err(InterpretError::Compile);
    //                 }
    //             }
    //             OpCode::Return => {
    //                 if let Some(val) = self.stack.pop_back() {
    //                     println!("{val}");
    //                     return Ok(());
    //                 }
    //                 eprintln!("Stack underflow.");
    //                 return Err(InterpretError::Compile);
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    fn binary_op<T>(&mut self, op: T) -> Result<(), StackIsEmptyError>
    where
        T: FnOnce(Value, Value) -> Value,
    {
        let b = self.stack.pop_back().ok_or(StackIsEmptyError)?;
        let a = self.stack.pop_back().ok_or(StackIsEmptyError)?;
        self.stack.push_back(op(a, b));
        Ok(())
    }
}

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

#[derive(Debug, Error)]
enum RuntimeError {
    #[error("Stack underflow.")]
    StackUnderflow,
    #[error("Operand must be a {0}")]
    InvalidOperand(&'static str),
}

#[derive(Debug, Default)]
pub struct Vm {
    stack: VecDeque<Value>,
    chunk: Option<Chunk>,
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
    pub fn repl(&mut self) -> io::Result<()> {
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;

            if line.is_empty() {
                break Ok(());
            }

            self.interpret(line);
        }
    }

    #[inline]
    pub fn run_file<P>(&mut self, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(path)?;

        self.interpret(contents);

        Ok(())
    }

    #[inline]
    fn interpret<S>(&mut self, source: S)
    where
        S: AsRef<str>,
    {
        let mut chunk = Chunk::new();

        if let Err(err) = compiler::compile(source.as_ref(), &mut chunk) {
            eprintln!("{err}");
        }

        self.chunk = Some(chunk);

        let result = self.run();

        if let Err(err) = result {
            eprintln!("{err}");
            let line = self
                .chunk
                .as_ref()
                .map_or(&0, |chunk| chunk.lines.first().unwrap_or(&0));
            eprintln!("[line {line}] in script");
        }
    }

    fn run(&mut self) -> Result<(), RuntimeError> {
        if let Some(chunk) = self.chunk.take() {
            for code in chunk.codes {
                match code {
                    OpCode::Constant(const_idx) => {
                        let constant = chunk
                            .constants
                            .get(const_idx)
                            .ok_or(RuntimeError::StackUnderflow)?;
                        self.stack.push_back(*constant);
                    }
                    OpCode::Nil => {
                        self.stack.push_back(Value::Nil);
                    }
                    OpCode::True => {
                        self.stack.push_back(Value::Bool(true));
                    }
                    OpCode::False => {
                        self.stack.push_back(Value::Bool(false));
                    }
                    OpCode::Equal => {
                        let a = self
                            .stack
                            .pop_back()
                            .ok_or(RuntimeError::StackUnderflow)?;
                        let b = self
                            .stack
                            .pop_back()
                            .ok_or(RuntimeError::StackUnderflow)?;
                        self.stack.push_back(Value::Bool(a == b));
                    }
                    OpCode::Greater => {
                        self.binary_op(|a, b| Value::Bool(a > b))?;
                    }
                    OpCode::Less => {
                        self.binary_op(|a, b| Value::Bool(a < b))?;
                    }
                    OpCode::Add => {
                        self.binary_op(|a, b| Value::Number(a + b))?;
                    }
                    OpCode::Subtract => {
                        self.binary_op(|a, b| Value::Number(a - b))?;
                    }
                    OpCode::Multiply => {
                        self.binary_op(|a, b| Value::Number(a * b))?;
                    }
                    OpCode::Divide => {
                        self.binary_op(|a, b| Value::Number(a / b))?;
                    }
                    OpCode::Not => {
                        let value = self
                            .stack
                            .pop_back()
                            .ok_or(RuntimeError::StackUnderflow)?
                            .is_falsey();
                        self.stack.push_back(Value::Bool(value));
                    }
                    OpCode::Negate => {
                        let value = self
                            .stack
                            .pop_back()
                            .ok_or(RuntimeError::StackUnderflow)?;
                        let number = value
                            .as_number()
                            .ok_or(RuntimeError::InvalidOperand("number"))?;
                        self.stack.push_back(Value::Number(-number));
                    }
                    OpCode::Return => {
                        let value = self
                            .stack
                            .pop_back()
                            .ok_or(RuntimeError::StackUnderflow)?;
                        println!("{value}");
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    fn binary_op<T>(&mut self, op: T) -> Result<(), RuntimeError>
    where
        T: FnOnce(f64, f64) -> Value,
    {
        let b = self
            .stack
            .pop_back()
            .ok_or(RuntimeError::StackUnderflow)?
            .as_number()
            .ok_or(RuntimeError::InvalidOperand("number"))?;

        let a = self
            .stack
            .pop_back()
            .ok_or(RuntimeError::StackUnderflow)?
            .as_number()
            .ok_or(RuntimeError::InvalidOperand("number"))?;

        self.stack.push_back(op(a, b));

        Ok(())
    }
}

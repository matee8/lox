use core::ops::{Add, Div, Mul, Sub};
use std::collections::VecDeque;

use thiserror::Error;

use crate::{
    chunk::{Chunk, OpCode},
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

#[derive(Debug, Error)]
#[error("Stack is empty.")]
struct StackIsEmptyError;

#[derive(Debug)]
pub struct Vm<'chunk> {
    chunk: &'chunk Chunk,
    stack: VecDeque<Value>,
}

impl<'chunk> Vm<'chunk> {
    #[inline]
    #[must_use]
    pub fn new(chunk: &'chunk Chunk) -> Self {
        Self {
            chunk,
            stack: VecDeque::with_capacity(256),
        }
    }

    #[inline]
    pub fn interpret(&mut self) -> Result<(), InterpretError> {
        for code in self.chunk.codes() {
            match *code {
                OpCode::Constant(const_idx) => {
                    let Some(constant) = self.chunk.constants().get(const_idx)
                    else {
                        eprintln!("Stack underflow.");
                        return Err(InterpretError::Compile);
                    };
                    self.stack.push_back(*constant);
                }
                OpCode::Add => {
                    if self.binary_op(Add::add).is_err() {
                        eprintln!("Stack underflow.");
                        return Err(InterpretError::Compile);
                    }
                }
                OpCode::Subtract => {
                    if self.binary_op(Sub::sub).is_err() {
                        eprintln!("Stack underflow");
                        return Err(InterpretError::Compile);
                    }
                }
                OpCode::Multiply => {
                    if self.binary_op(Mul::mul).is_err() {
                        eprintln!("Stack underflow");
                        return Err(InterpretError::Compile);
                    }
                }
                OpCode::Divide => {
                    if self.binary_op(Div::div).is_err() {
                        eprintln!("Stack underflow");
                        return Err(InterpretError::Compile);
                    }
                }
                OpCode::Negate => {
                    if let Some(val) = self.stack.pop_back() {
                        self.stack.push_back(-val);
                    } else {
                        eprintln!("Stack underflow.");
                        return Err(InterpretError::Compile);
                    }
                }
                OpCode::Return => {
                    if let Some(val) = self.stack.pop_back() {
                        println!("{val}");
                        return Ok(());
                    }
                    eprintln!("Stack underflow.");
                    return Err(InterpretError::Compile);
                }
            }
        }
        Ok(())
    }

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

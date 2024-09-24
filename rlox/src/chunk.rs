use core::fmt::{self, Debug, Formatter};

use crate::value::Value;

#[non_exhaustive]
#[derive(Debug)]
pub enum OpCode {
    Constant(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

#[derive(Default)]
pub struct Chunk {
    codes: Vec<OpCode>,
    constants: Vec<Value>,
    lines: Vec<i32>,
}

impl Chunk {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self {
            codes: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    #[inline]
    pub fn write_opcode(&mut self, code: OpCode, line: i32) {
        self.codes.push(code);
        self.lines.push(line);
    }

    #[inline]
    pub fn write_constant(&mut self, constant: Value, line: i32) {
        self.constants.push(constant);
        self.write_opcode(OpCode::Constant(self.constants.len() - 1), line);
    }

    pub(crate) const fn codes(&self) -> &Vec<OpCode> {
        &self.codes
    }

    pub(crate) const fn constants(&self) -> &Vec<Value> {
        &self.constants
    }
}

impl Debug for Chunk {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "== CHUNK ==")?;
        for (i, instruction) in self.codes.iter().enumerate() {
            write!(f, "{i:04} ")?;

            let line = self.lines[i];
            if i > 0 && line == self.lines[i - 1] {
                write!(f, "   | ")?;
            } else {
                write!(f, "{line:04} ")?;
            }

            match *instruction {
                OpCode::Constant(const_idx) => {
                    let const_val = self.constants[const_idx];
                    writeln!(
                        f,
                        "{:<16} {const_idx:4} {const_val}",
                        "OP_CONSTANT"
                    )?;
                }
                OpCode::Add => writeln!(f, "OP_ADD")?,
                OpCode::Subtract => writeln!(f, "OP_SUBTRACT")?,
                OpCode::Multiply => writeln!(f, "OP_MULTIPLY")?,
                OpCode::Divide => writeln!(f, "OP_DIVIDE")?,
                OpCode::Negate => writeln!(f, "OP_NEGATE")?,
                OpCode::Return => writeln!(f, "OP_RETURN")?,
            }
        }
        Ok(())
    }
}

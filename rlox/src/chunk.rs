use core::fmt::{self, Debug, Formatter};

use crate::value::Value;

#[non_exhaustive]
#[derive(Debug)]
pub enum OpCode {
    OpConstant(usize),
    OpReturn,
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
        self.write_opcode(OpCode::OpConstant(self.constants.len() - 1), line);
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
                OpCode::OpConstant(const_idx) => {
                    let const_val = self.constants[const_idx];
                    writeln!(
                        f,
                        "{:<16} {const_idx:4} {const_val}",
                        "OP_CONSTANT"
                    )?;
                }
                OpCode::OpReturn => writeln!(f, "OP_RETURN")?,
            }
        }
        Ok(())
    }
}

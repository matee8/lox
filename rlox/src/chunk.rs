use core::fmt::{self, Debug, Formatter};

use crate::value::Value;

pub enum OpCode {
    Constant(usize),
    Nil,
    True,
    False,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Return,
}

pub struct Chunk {
    pub codes: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub const fn new() -> Self {
        Self {
            codes: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_opcode(&mut self, code: OpCode, line: i32) {
        self.codes.push(code);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, constant: Value, line: i32) {
        self.constants.push(constant);
        self.write_opcode(OpCode::Constant(self.constants.len() - 1), line);
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "== CHUNK ==")?;
        for (i, instruction) in self.codes.iter().enumerate() {
            write!(f, "{i:04} ")?;

            #[expect(
                clippy::indexing_slicing,
                reason = r#"
                    `self.codes` and `self.lines` vectors are always modified
                    together via `self.write_opcode()`.
                "#
            )]
            let line = self.lines[i];
            #[expect(
                clippy::indexing_slicing,
                reason = r#"
                    `self.codes` and `self.lines` always have the same length
                    (`i`), validated through enumeration.
                "#
            )]
            if i > 0 && line == self.lines[i - 1] {
                write!(f, "   | ")?;
            } else {
                write!(f, "{line:04} ")?;
            }

            match *instruction {
                OpCode::Constant(const_idx) => {
                    #[expect(
                        clippy::indexing_slicing,
                        reason = r#"
                            Constant indexes are always valid as they come from
                            `self.write_constant()`.
                        "#
                    )]
                    let const_val = &self.constants[const_idx];
                    writeln!(
                        f,
                        "{:<16} {const_idx:4} {const_val}",
                        "OP_CONSTANT"
                    )?;
                }
                OpCode::Nil => writeln!(f, "OP_NIL")?,
                OpCode::True => writeln!(f, "OP_TRUE")?,
                OpCode::False => writeln!(f, "OP_TRUE")?,
                OpCode::Equal => writeln!(f, "OP_EQUAL")?,
                OpCode::Greater => writeln!(f, "OP_GREATER")?,
                OpCode::Less => writeln!(f, "OP_LESS")?,
                OpCode::Add => writeln!(f, "OP_ADD")?,
                OpCode::Subtract => writeln!(f, "OP_SUBTRACT")?,
                OpCode::Multiply => writeln!(f, "OP_MULTIPLY")?,
                OpCode::Divide => writeln!(f, "OP_DIVIDE")?,
                OpCode::Not => writeln!(f, "OP_NOT")?,
                OpCode::Negate => writeln!(f, "OP_NEGATE")?,
                OpCode::Return => writeln!(f, "OP_RETURN")?,
            }
        }
        Ok(())
    }
}

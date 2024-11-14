use core::num::ParseFloatError;

use thiserror::Error;

use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
};

struct Parser<'src> {
    current: Option<Token<'src>>,
    previous: Option<Token<'src>>,
    had_error: bool,
    panic_mode: bool,
}

enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

enum ErrorLocation {
    Current,
    Previous,
}

impl<'src> Parser<'src> {
    const fn new() -> Self {
        Self {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "ErrorLocation is only a compile time data, no reason to care about references."
    )]
    fn error(&mut self, location: ErrorLocation, msg: &'src str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        let token = match location {
            ErrorLocation::Current => &self.current,
            ErrorLocation::Previous => &self.previous,
        };

        if let Some(ref token) = *token {
            eprint!("[line {}] Error", token.line);

            #[expect(
                clippy::else_if_without_else,
                reason = "If the error is not at the end or at a specific line, we can't specify the location."
            )]
            if token.r#type == TokenType::Eof {
                eprint!(" at end");
            } else if token.r#type != TokenType::Error {
                eprint!(" at {}", token.lexeme);
            }
        } else {
            eprint!("Error");
        }

        eprintln!(": {msg}");
        self.had_error = true;
    }

    fn advance(&mut self, scanner: &mut Scanner<'src>) {
        self.previous = self.current.take();

        loop {
            let current = scanner.scan_token();

            if current.r#type != TokenType::Error {
                self.current = Some(current);
                break;
            }

            self.error(ErrorLocation::Current, current.lexeme);
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "Type argument is always constructed when calling this function, it never exists before."
    )]
    fn consume(
        &mut self,
        scanner: &mut Scanner<'src>,
        r#type: TokenType,
        msg: &'src str,
    ) {
        if let Some(current) = self.current.as_ref() {
            if current.r#type == r#type {
                self.advance(scanner);
            } else {
                self.error(ErrorLocation::Current, msg);
            }
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "Precedence argument is always constructed when calling this function, it never exists before."
    )]
    fn parse_precedence(&self, prcedence: Precedence) {}

    fn expression(&self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn unary(&self, chunk: &mut Chunk) {
        if let Some(ref previous) = self.previous {
            let op_type = &previous.r#type;

            self.parse_precedence(Precedence::Unary);

            match *op_type {
                TokenType::Minus => {
                    chunk.write_opcode(OpCode::Negate, previous.line);
                }
                _ => {},
            }
        }
    }

    fn grouping(&mut self, scanner: &mut Scanner<'src>) {
        self.expression();
        self.consume(
            scanner,
            TokenType::RightParen,
            "Expect ')' after expression.",
        );
    }

    fn number(&self, chunk: &mut Chunk) -> Result<(), ParseFloatError> {
        if let Some(ref previous) = self.previous {
            let value: f64 = previous.lexeme.parse()?;
            chunk.write_constant(value, previous.line);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
#[error("Failed to compile code.")]
pub struct CompilerError;

#[inline]
pub fn compile<S>(source: S, chunk: &mut Chunk) -> Result<(), CompilerError>
where
    S: AsRef<str>,
{
    let mut scanner = Scanner::new(source.as_ref());
    let mut parser = Parser::new();

    parser.advance(&mut scanner);
    parser.expression();
    parser.consume(&mut scanner, TokenType::Eof, "Expect end of expression.");

    if let Some(previous) = parser.previous {
        chunk.write_opcode(OpCode::Return, previous.line);
    } else {
        parser.had_error = true;
    }

    if parser.had_error {
        Err(CompilerError)
    } else {
        Ok(())
    }
}

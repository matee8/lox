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
    scanner: &'src mut Scanner<'src>,
    chunk: &'src mut Chunk,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
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

impl Precedence {
    #[inline]
    #[must_use]
    pub const fn next_level(&self) -> Option<Self> {
        match *self {
            Self::None => Some(Self::Assignment),
            Self::Assignment => Some(Self::Or),
            Self::Or => Some(Self::And),
            Self::And => Some(Self::Equality),
            Self::Equality => Some(Self::Comparison),
            Self::Comparison => Some(Self::Term),
            Self::Term => Some(Self::Factor),
            Self::Factor => Some(Self::Unary),
            Self::Unary => Some(Self::Call),
            Self::Call => Some(Self::Primary),
            Self::Primary => None,
        }
    }
}

enum ParseFn {
    Grouping,
    Unary,
    Binary,
    Number,
}

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

enum ErrorLocation {
    Current,
    Previous,
}

impl<'src> Parser<'src> {
    const fn get_rule(r#type: TokenType) -> ParseRule {
        match r#type {
            TokenType::RightParen
            | TokenType::LeftBrace
            | TokenType::RightBrace
            | TokenType::Comma
            | TokenType::Dot
            | TokenType::Semicolon
            | TokenType::Bang
            | TokenType::BangEqual
            | TokenType::Equal
            | TokenType::EqualEqual
            | TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual
            | TokenType::Identifier
            | TokenType::String
            | TokenType::And
            | TokenType::Class
            | TokenType::Else
            | TokenType::False
            | TokenType::For
            | TokenType::Fun
            | TokenType::If
            | TokenType::Nil
            | TokenType::Or
            | TokenType::Print
            | TokenType::Return
            | TokenType::Super
            | TokenType::This
            | TokenType::True
            | TokenType::Var
            | TokenType::While
            | TokenType::Error
            | TokenType::Eof => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::LeftParen => ParseRule {
                prefix: Some(ParseFn::Grouping),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Minus => ParseRule {
                prefix: Some(ParseFn::Unary),
                infix: Some(ParseFn::Binary),
                precedence: Precedence::Term,
            },
            TokenType::Plus => ParseRule {
                prefix: None,
                infix: Some(ParseFn::Binary),
                precedence: Precedence::Term,
            },
            TokenType::Slash | TokenType::Star => ParseRule {
                prefix: None,
                infix: Some(ParseFn::Binary),
                precedence: Precedence::Factor,
            },
            TokenType::Number => ParseRule {
                prefix: Some(ParseFn::Number),
                infix: None,
                precedence: Precedence::None,
            },
        }
    }

    const fn new(
        scanner: &'src mut Scanner<'src>,
        chunk: &'src mut Chunk,
    ) -> Self {
        Self {
            current: None,
            previous: None,
            had_error: false,
            panic_mode: false,
            scanner,
            chunk,
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

    fn advance(&mut self) {
        self.previous = self.current.take();

        loop {
            let current = self.scanner.scan_token();

            if current.r#type != TokenType::Error {
                self.current = Some(current);
                break;
            }

            self.error(ErrorLocation::Current, current.lexeme);
        }
    }

    fn consume(&mut self, r#type: TokenType, msg: &'src str) {
        if let Some(current) = self.current.as_ref() {
            if current.r#type == r#type {
                self.advance();
            } else {
                self.error(ErrorLocation::Current, msg);
            }
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "Precedence argument is always constructed when calling this function, it never exists before."
    )]
    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        if let Some(ref previous) = self.previous {
            let Some(prefix_rule) = Self::get_rule(previous.r#type).prefix
            else {
                self.error(ErrorLocation::Current, "Expect expression.");
                return;
            };

            match prefix_rule {
                ParseFn::Unary => {
                    self.unary();
                }
                ParseFn::Binary => {
                    self.binary();
                }
                ParseFn::Grouping => {
                    self.grouping();
                }
                ParseFn::Number => {
                    if self.number().is_err() {
                        self.error(ErrorLocation::Current, "Invalid number.");
                    }
                }
            }

            while let Some(ref current) = self.current {
                let rule = Self::get_rule(current.r#type);
                if rule.precedence < precedence {
                    break;
                }

                self.advance();

                if let Some(infix_rule) = rule.infix {
                    match infix_rule {
                        ParseFn::Unary => {
                            self.unary();
                        }
                        ParseFn::Binary => {
                            self.binary();
                        }
                        ParseFn::Grouping => {
                            self.grouping();
                        }
                        ParseFn::Number => {
                            if self.number().is_err() {
                                self.error(
                                    ErrorLocation::Current,
                                    "Invalid number.",
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn unary(&mut self) {
        if let Some(ref previous) = self.previous {
            let op_type = previous.r#type;
            let line = previous.line;

            self.parse_precedence(Precedence::Unary);

            if op_type == TokenType::Minus {
                self.chunk.write_opcode(OpCode::Negate, line);
            }
        }
    }

    fn binary(&mut self) {
        if let Some(ref previous) = self.previous {
            let op_type = previous.r#type;
            let rule = Self::get_rule(op_type);
            let line = previous.line;

            let Some(next_level) = rule.precedence.next_level() else {
                self.error(ErrorLocation::Current, "???");
                return;
            };

            self.parse_precedence(next_level);

            match op_type {
                TokenType::Plus => {
                    self.chunk.write_opcode(OpCode::Add, line);
                }
                TokenType::Minus => {
                    self.chunk.write_opcode(OpCode::Subtract, line);
                }
                TokenType::Star => {
                    self.chunk.write_opcode(OpCode::Multiply, line);
                }
                TokenType::Slash => {
                    self.chunk.write_opcode(OpCode::Divide, line);
                }
                _ => {}
            }
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) -> Result<(), ParseFloatError> {
        if let Some(ref previous) = self.previous {
            let value: f64 = previous.lexeme.parse()?;
            self.chunk.write_constant(value, previous.line);
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
    let mut parser = Parser::new(&mut scanner, chunk);

    parser.advance();
    parser.expression();
    parser.consume(TokenType::Eof, "Expect end of expression.");

    if let Some(previous) = parser.previous {
        parser.chunk.write_opcode(OpCode::Return, previous.line);
    } else {
        parser.had_error = true;
    }

    if parser.had_error {
        Err(CompilerError)
    } else {
        Ok(())
    }
}

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

#[derive(PartialEq, PartialOrd, Eq, Ord)]
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
    fn parse_precedence(
        &mut self,
        scanner: &mut Scanner<'src>,
        chunk: &mut Chunk,
        precedence: Precedence,
    ) {
        self.advance(scanner);
        if let Some(ref previous) = self.previous {
            let Some(prefix_rule) = Self::get_rule(&previous.r#type).prefix else {
                self.error(ErrorLocation::Current, "Expect expression.");
                return;
            };

            match prefix_rule {
                ParseFn::Unary => {
                    self.unary(chunk);
                }
                ParseFn::Binary => {
                    self.binary(chunk);
                }
                ParseFn::Grouping => {
                    self.grouping(scanner);
                }
                ParseFn::Number => {
                    self.number(chunk);
                }
            }

            if let Some(current) = self.current {
                while precedence <= Self::get_rule(&current.r#type).precedence {
                    self.advance(scanner);
                    let infix_rule = Self::get_rule(&previous.r#type).infix;
                    match infix_rule {
                        Some(rule) => match rule {
                            ParseFn::Unary => {
                                self.unary(chunk);
                            }
                            ParseFn::Binary => {
                                self.binary(chunk);
                            }
                            ParseFn::Grouping => {
                                self.grouping(scanner);
                            }
                            ParseFn::Number => {
                                self.number(chunk);
                            }
                        }
                        None => {}
                    }
                }
            }
        }
    }

    const fn get_rule(r#type: &TokenType) -> ParseRule {
        match *r#type {
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
                _ => {}
            }
        }
    }

    fn binary(&self, chunk: &mut Chunk) {
        if let Some(ref previous) = self.previous {
            let op_type = &previous.r#type;
            // let rule = get_rule(op_type);

            // self.parse_precedence((rule.precedence + 1) as Precedence);

            match *op_type {
                TokenType::Plus => {
                    chunk.write_opcode(OpCode::Add, previous.line);
                }
                TokenType::Minus => {
                    chunk.write_opcode(OpCode::Negate, previous.line);
                }
                TokenType::Star => {
                    chunk.write_opcode(OpCode::Multiply, previous.line);
                }
                TokenType::Slash => {
                    chunk.write_opcode(OpCode::Divide, previous.line);
                }
                _ => {}
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

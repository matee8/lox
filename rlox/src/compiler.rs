use thiserror::Error;

use crate::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
};

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
    const fn next_level(&self) -> Option<Self> {
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

#[derive(Debug, Error)]
pub enum ParserError<'src> {
    #[error("[line {line}] Error at {location}: {msg}")]
    AtToken {
        line: i32,
        location: &'src str,
        msg: &'src str,
    },
    #[error("[line {line}] Error: {msg}")]
    General { line: i32, msg: &'src str },
}

enum ParseState {
    Prefix,
    Infix,
    Done,
}

struct Parser<'src, 'scanner> {
    current: Option<Token<'src>>,
    previous: Option<Token<'src>>,
    scanner: &'scanner mut Scanner<'src>,
    chunk: &'src mut Chunk,
}

impl<'src, 'scanner> Parser<'src, 'scanner> {
    #[expect(
        clippy::as_conversions,
        clippy::indexing_slicing,
        reason = r#"
            `TokenType` discriminants are contiguous from 0, and `rules` array
            length exactly matches the number of `TokenType` variants.
        "#
    )]
    const LOOKUP_RULES: [ParseRule; 40] = {
        const DEFAULT: ParseRule = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        let mut rules = [DEFAULT; TokenType::Eof as usize + 1];

        rules[TokenType::LeftParen as usize] = ParseRule {
            prefix: Some(ParseFn::Grouping),
            infix: None,
            precedence: Precedence::None,
        };
        rules[TokenType::Minus as usize] = ParseRule {
            prefix: Some(ParseFn::Unary),
            infix: Some(ParseFn::Binary),
            precedence: Precedence::Term,
        };
        rules[TokenType::Plus as usize] = ParseRule {
            prefix: None,
            infix: Some(ParseFn::Binary),
            precedence: Precedence::Term,
        };
        rules[TokenType::Slash as usize] = ParseRule {
            prefix: None,
            infix: Some(ParseFn::Binary),
            precedence: Precedence::Factor,
        };
        rules[TokenType::Star as usize] = ParseRule {
            prefix: None,
            infix: Some(ParseFn::Binary),
            precedence: Precedence::Factor,
        };
        rules[TokenType::Number as usize] = ParseRule {
            prefix: Some(ParseFn::Number),
            infix: None,
            precedence: Precedence::None,
        };

        rules
    };

    const fn new(
        scanner: &'scanner mut Scanner<'src>,
        chunk: &'src mut Chunk,
    ) -> Self {
        Self {
            current: None,
            previous: None,
            scanner,
            chunk,
        }
    }

    #[expect(
        clippy::indexing_slicing,
        reason = r#"
             `LOOKUP_RULES` array size is explicitly set to the number of
             `TokenType` discriminant values. All enum variants are covered
             in array initialization.
        "#
    )]
    const fn get_rule(r#type: TokenType) -> &'static ParseRule {
        &Self::LOOKUP_RULES[r#type as usize]
    }

    fn advance(&mut self) -> Result<(), ParserError<'src>> {
        self.previous = self.current.take();

        let current = self.scanner.scan_token();

        if current.r#type == TokenType::Error {
            Err(ParserError::AtToken {
                line: current.line,
                location: current.lexeme,
                msg: current.lexeme,
            })
        } else {
            self.current = Some(current);
            Ok(())
        }
    }

    fn consume(
        &mut self,
        r#type: TokenType,
        msg: &'src str,
    ) -> Result<(), ParserError<'src>> {
        let current = self.current.as_ref().ok_or(ParserError::AtToken {
            line: 0,
            location: "end of input",
            msg,
        })?;

        if current.r#type == r#type {
            self.advance()?;
            Ok(())
        } else {
            Err(ParserError::AtToken {
                line: current.line,
                location: current.lexeme,
                msg,
            })
        }
    }

    fn parse_precedence(
        &mut self,
        precedence: &Precedence,
    ) -> Result<(), ParserError<'src>> {
        self.advance()?;
        let mut state = ParseState::Prefix;
        loop {
            match state {
                ParseState::Prefix => {
                    let previous =
                        self.previous.as_ref().ok_or(ParserError::General {
                            line: 0,
                            msg: "No previous token in prefix state.",
                        })?;
                    let rule = Self::get_rule(previous.r#type);
                    let prefix_rule =
                        rule.prefix.as_ref().ok_or(ParserError::AtToken {
                            line: previous.line,
                            location: previous.lexeme,
                            msg: "Expect expression.",
                        })?;
                    match *prefix_rule {
                        ParseFn::Unary => self.unary()?,
                        ParseFn::Binary => self.binary()?,
                        ParseFn::Grouping => self.grouping()?,
                        ParseFn::Number => self.number()?,
                    }
                    state = ParseState::Infix;
                }
                ParseState::Infix => {
                    let Some(current) = self.current.as_ref() else {
                        state = ParseState::Done;
                        continue;
                    };

                    let rule = Self::get_rule(current.r#type);

                    if rule.precedence < *precedence {
                        state = ParseState::Done;
                        continue;
                    }

                    self.advance()?;

                    let Some(infix_rule) = rule.infix.as_ref() else {
                        state = ParseState::Done;
                        continue;
                    };

                    match *infix_rule {
                        ParseFn::Unary => self.unary()?,
                        ParseFn::Binary => self.binary()?,
                        ParseFn::Grouping => self.grouping()?,
                        ParseFn::Number => self.number()?,
                    }
                }
                ParseState::Done => break Ok(()),
            }
        }
    }

    fn expression(&mut self) -> Result<(), ParserError<'src>> {
        self.parse_precedence(&Precedence::Assignment)
    }

    fn unary(&mut self) -> Result<(), ParserError<'src>> {
        let previous = self.previous.as_ref().ok_or(ParserError::General {
            line: 0,
            msg: "No previous token in binary.",
        })?;
        let op_type = previous.r#type;
        let line = previous.line;

        self.parse_precedence(&Precedence::Unary)?;

        if matches!(op_type, TokenType::Minus) {
            self.chunk.write_opcode(OpCode::Negate, line);
        }

        Ok(())
    }

    fn binary(&mut self) -> Result<(), ParserError<'src>> {
        let previous = self.previous.as_ref().ok_or(ParserError::General {
            line: 0,
            msg: "No previous token in binary.",
        })?;
        let op_type = previous.r#type;
        let line = previous.line;
        let rule = Self::get_rule(op_type);
        let next_precedence =
            rule.precedence.next_level().ok_or(ParserError::General {
                line,
                msg: "Missing next precedence level.",
            })?;

        self.parse_precedence(&next_precedence)?;

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

        Ok(())
    }

    fn grouping(&mut self) -> Result<(), ParserError<'src>> {
        self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after expression.")
    }

    fn number(&mut self) -> Result<(), ParserError<'src>> {
        let previous = self.previous.as_ref().ok_or(ParserError::General {
            line: 0,
            msg: "No previous token in binary.",
        })?;

        #[expect(
            clippy::map_err_ignore,
            reason = r#"
                Specific parse errors contain details not usefor for end users.
            "#
        )]
        let value: f64 =
            previous.lexeme.parse().map_err(|_| ParserError::AtToken {
                line: previous.line,
                location: previous.lexeme,
                msg: "Invalid number.",
            })?;
        self.chunk.write_constant(value, previous.line);

        Ok(())
    }
}

#[inline]
pub fn compile<'src>(
    source: &'src str,
    chunk: &'src mut Chunk,
) -> Result<(), ParserError<'src>> {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new(&mut scanner, chunk);

    parser.advance()?;
    parser.expression()?;
    parser.consume(TokenType::Eof, "Expect end of expression.")?;

    let previous = parser.previous.ok_or(ParserError::General {
        line: 0,
        msg: "Failed to compile code.",
    })?;

    parser.chunk.write_opcode(OpCode::Return, previous.line);
    Ok(())
}

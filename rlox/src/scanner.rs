#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token<'lexeme> {
    pub r#type: TokenType,
    pub lexeme: &'lexeme str,
    pub line: i32,
}

#[derive(Debug, Clone)]
pub struct Scanner<'src> {
    pub source: &'src str,
    pub start: usize,
    pub current: usize,
    pub line: i32,
}

impl<'src> Scanner<'src> {
    #[must_use]
    #[inline]
    pub const fn new(source: &'src str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    #[inline]
    pub fn scan_token(&mut self) -> Token<'src> {
        self.start = self.current;

        if self.is_at_end() {
            self.make_token(TokenType::Eof)
        } else {
            self.error_token("Unexpected character.")
        }
    }

    const fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, r#type: TokenType) -> Token<'src> {
        Token {
            r#type,
            lexeme: &self.source[self.start..self.current],
            line: self.line,
        }
    }

    const fn error_token(&self, message: &'static str) -> Token<'static> {
        Token {
            r#type: TokenType::Error,
            lexeme: message,
            line: self.line,
        }
    }
}

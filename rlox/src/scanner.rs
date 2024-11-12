#[derive(PartialEq, Eq)]
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

pub struct Token<'lexeme> {
    pub r#type: TokenType,
    pub lexeme: &'lexeme str,
    pub line: i32,
}

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
        self.skip_whitespace();
        self.start = self.current;

        let Some(char) = self.advance() else {
            return self.make_token(TokenType::Eof);
        };

        match char {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' if self.matches('=') => self.make_token(TokenType::BangEqual),
            '!' => self.make_token(TokenType::Bang),
            '=' if self.matches('=') => self.make_token(TokenType::EqualEqual),
            '=' => self.make_token(TokenType::Equal),
            '<' if self.matches('=') => self.make_token(TokenType::LessEqual),
            '<' => self.make_token(TokenType::Less),
            '>' if self.matches('=') => {
                self.make_token(TokenType::GreaterEqual)
            }
            '>' => self.make_token(TokenType::Greater),
            '"' => self.string(),
            '0'..='9' => self.number(),
            char if char.is_alphabetic() => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn matches(&mut self, expected: char) -> bool {
        self.advance().map_or(false, |char| char == expected)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\r' | '\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('/') if self.peek_next() == Some('/') => {
                    while self.peek().is_some() {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn make_token(&self, r#type: TokenType) -> Token<'src> {
        #[expect(
            clippy::string_slice,
            reason = "self.start and self.current are only modified by self, so it's safe to index."
        )]
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

    fn check_keyword(
        &self,
        start: usize,
        rest: &str,
        r#type: TokenType,
    ) -> TokenType {
        #[expect(
            clippy::string_slice,
            reason = "self.start and self.current are only modified by self, so it's safe to index."
        )]
        if self.current - self.start == start + rest.len()
            && rest == &self.source[self.start + start..rest.len()]
        {
            r#type
        } else {
            TokenType::Identifier
        }
    }

    fn string(&mut self) -> Token<'src> {
        while self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.peek().is_none() {
            self.error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn number(&mut self) -> Token<'src> {
        while self.peek().is_some_and(char::is_numeric) {
            self.advance();
        }

        if self.peek() == Some('.')
            && self.peek_next().is_some_and(char::is_numeric)
        {
            self.advance();

            while self.peek().is_some_and(char::is_numeric) {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token<'src> {
        while self.peek().is_some_and(char::is_alphabetic) {
            self.advance();
        }

        let identifier_type = match self.source.chars().nth(self.start) {
            None => {
                return self.error_token("Unexpected character.");
            }
            Some(char) => match char {
                'a' => self.check_keyword(1, "nd", TokenType::And),
                'c' => self.check_keyword(1, "lass", TokenType::Class),
                'e' => self.check_keyword(1, "lse", TokenType::Else),
                'i' => self.check_keyword(1, "f", TokenType::If),
                'n' => self.check_keyword(1, "il", TokenType::Nil),
                'o' => self.check_keyword(1, "r", TokenType::Or),
                'p' => self.check_keyword(1, "rint", TokenType::Print),
                'r' => self.check_keyword(1, "eturn", TokenType::Return),
                's' => self.check_keyword(1, "uper", TokenType::Super),
                'v' => self.check_keyword(1, "ar", TokenType::Var),
                'w' => self.check_keyword(1, "hile", TokenType::While),
                'f' => {
                    if self.current - self.start > 1 {
                        match self.source.chars().nth(self.start + 1) {
                            Some('a') => {
                                self.check_keyword(2, "lse", TokenType::False)
                            }
                            Some('o') => {
                                self.check_keyword(2, "r", TokenType::For)
                            }
                            Some('u') => {
                                self.check_keyword(2, "n", TokenType::Fun)
                            }
                            _ => TokenType::Identifier,
                        }
                    } else {
                        TokenType::Identifier
                    }
                }
                't' => {
                    if self.current - self.start > 1 {
                        match self.source.chars().nth(self.start + 1) {
                            Some('h') => {
                                self.check_keyword(2, "is", TokenType::This)
                            }
                            Some('r') => {
                                self.check_keyword(2, "ue", TokenType::True)
                            }
                            _ => TokenType::Identifier,
                        }
                    } else {
                        TokenType::Identifier
                    }
                }
                _ => TokenType::Identifier,
            },
        };

        self.make_token(identifier_type)
    }
}

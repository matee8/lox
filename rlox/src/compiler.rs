use crate::scanner::{Scanner, TokenType};

#[inline]
pub fn compile<S>(source: S)
where
    S: AsRef<str>,
{
    let mut scanner = Scanner::new(source.as_ref());

    let mut line = -1;

    loop {
        let token = scanner.scan_token();
        if token.line == line {
            println!("   | ");
        } else {
            print!("{:04} ", token.line);
            line = token.line;
        }
        println!("{:2?} '{}'", token.r#type, token.lexeme);

        if token.r#type == TokenType::Eof {
            break;
        }
    }
}

use thiserror::Error;

use crate::scanner::Scanner;

#[derive(Debug, Error)]
#[error("Failed to compile code.")]
pub struct CompilerError;

#[inline]
pub fn compile<S>(source: S) -> Result<(), CompilerError>
where
    S: AsRef<str>,
{
    let mut _scanner = Scanner::new(source.as_ref());

    // advance();
    // expression();
    // consume(TokenType::Eof, "Expect end of expression.");

    Ok(())
}

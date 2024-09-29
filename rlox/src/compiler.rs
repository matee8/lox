use crate::scanner::Scanner;

#[inline]
pub fn compile<S>(source: S)
where
    S: AsRef<str>,
{
    let scanner = Scanner::new(source.as_ref());
}

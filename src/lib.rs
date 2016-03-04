extern crate marksman_escape;

use marksman_escape::Escape;
use std::io::Write;
use std::io;

pub struct Escaped<T: AsRef<str>> {
    inner: T
}

pub struct SafeBuffer<T: Write> {
    inner: T
}

impl<T: Write> SafeBuffer<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T: Write> SafeBuffer<T> {
    pub fn new(inner: T) -> SafeBuffer<T> {
        SafeBuffer { inner: inner }
    }
}

pub trait Unescaped {
    fn escape(&self) -> Escaped<String>;
}

pub trait SafeWrite<T> {
    fn write_all(&mut self, buffer: T) -> io::Result<()>;
}

impl<T: AsRef<str>> Unescaped for T {
    fn escape(&self) -> Escaped<String> {
        let s: &str = self.as_ref();
        let e = String::from_utf8(Escape::new(s.bytes()).collect()).unwrap();
        Escaped { inner: e }
    }
}

impl<X: AsRef<str>, W: Write> SafeWrite<Escaped<X>> for SafeBuffer<W> {
    fn write_all(&mut self, buffer: Escaped<X>) -> io::Result<()> {
        self.inner.write_all(buffer.inner.as_ref().as_ref())
    }
}

impl<'a, X: Unescaped, W: Write> SafeWrite<X> for SafeBuffer<W> {
    fn write_all(&mut self, buffer: X) -> io::Result<()> {
        self.inner.write_all(buffer.escape().inner.as_ref())
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;

    #[test]
    fn escape_unsafe_string() {
        let mut buffer = SafeBuffer::new(vec![]);
        let s = String::from("<hello>&world</hello>");

        buffer.write_all(s);
        assert_eq!("&lt;hello&gt;&amp;world&lt;/hello&gt;", String::from_utf8(buffer.inner).unwrap());
    }

    #[test]
    fn escape_unsafe_str() {
        let mut buffer = SafeBuffer::new(vec![]);
        let s = "<hello>&world</hello>";

        buffer.write_all(s);
        assert_eq!("&lt;hello&gt;&amp;world&lt;/hello&gt;", String::from_utf8(buffer.inner).unwrap());
    }

    #[test]
    fn escape_safe_str() {
        let mut buffer = SafeBuffer::new(vec![]);
        let s = "<hello>&world</hello>".escape();

        buffer.write_all(s);
        assert_eq!("&lt;hello&gt;&amp;world&lt;/hello&gt;", String::from_utf8(buffer.inner).unwrap());
    }
}

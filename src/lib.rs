mod encode;
mod entities;

use encode::encode_attribute;
use encode::encode_attribute_w;
use std::io::Write;
use std::io;

pub struct Escaped<T: AsRef<str>> {
    inner: T
}

impl<T: AsRef<str>> Escaped<T> {
    pub fn into_inner(self) -> T {
        self.inner
    }
}

pub trait Append<T> {
    fn append_str(self, string: T) -> Escaped<String>;
}

impl<T: AsRef<str>> Append<T> for Escaped<String> {
    fn append_str(mut self, string: T) -> Escaped<String> {
        self.inner.push_str(encode_attribute(string.as_ref()).as_ref());
        self
    }
}

impl<T: AsRef<str>> Append<Escaped<T>> for Escaped<String> {
    fn append_str(mut self, string: Escaped<T>) -> Escaped<String> {
        self.inner.push_str(string.into_inner().as_ref());
        self
    }
}

impl<T: AsRef<str>> Escaped<T> {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

pub struct SafeWriter<T: Write> {
    inner: T
}

impl<T: Write> SafeWriter<T> {
    pub fn new(inner: T) -> Self {
        SafeWriter { inner: inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

pub trait Escapable: AsRef<str> {
    fn escape(&self) -> Escaped<String>;
    fn safe(&self) -> Escaped<String>;
}

pub trait SafeWrite<T> {
    fn write_str(&mut self, buffer: T) -> io::Result<()>;
}

impl<T: AsRef<str>> Escapable for T {
    fn escape(&self) -> Escaped<String> {
        Escaped { inner: encode_attribute(self.as_ref()) }
    }

    fn safe(&self) -> Escaped<String> {
        Escaped { inner: self.as_ref().into() }
    }
}

impl<X: AsRef<str>, W: Write> SafeWrite<Escaped<X>> for SafeWriter<W> {
    fn write_str(&mut self, buffer: Escaped<X>) -> io::Result<()> {
        self.inner.write_all(buffer.as_ref().as_bytes())
    }
}

impl<'a, X: Escapable, W: Write> SafeWrite<X> for SafeWriter<W> {
    fn write_str(&mut self, buffer: X) -> io::Result<()> {
        encode_attribute_w(buffer.as_ref(), &mut self.inner)
    }
}


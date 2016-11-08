#![deny(missing_docs)]

//! escapade - type assisted html safety
//!
//!`escapade` is inspired by ActiveSupports SafeBuffer.
//!
//! `escapade` provides String concatenation and writing, but automatically escapes any HTML in the data in the process. This prevents accidental unescaped writes to the output.
//!
//! The library provides both a String type for HTML-safe concatenation and a writer, wrapping types implementing `Write`.
//!
//! The library works with any type that implements `AsRef<str>`.
mod encode;
mod entities;

use encode::encode_attribute;
use encode::encode_attribute_w;
use std::io::Write;
use std::io;

/// An escaped string-like value
///
/// Escaped wraps a value with the bounds `AsRef<str>`.
/// It can work on any of those values, but any operations
/// on them will return `Escaped<String>` and thus allocate.
pub struct Escaped<T: AsRef<str>> {
    inner: T
}

impl<T: AsRef<str>> Escaped<T> {
    /// Consumes the escaped marker and returns
    /// the wrapped value.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

/// Trait marking a value as appendable to `Escaped`
///
/// Values marked as `Append` can be appended to `Escaped`.
pub trait Append<T> {
    /// Append any string-like value
    fn append_str(&mut self, string: T);
}

impl<T: Escapable> Append<T> for Escaped<String> {
    fn append_str(&mut self, string: T) {
        self.append_str(string.escape())
    }
}

impl<T: AsRef<str>> Append<Escaped<T>> for Escaped<String> {
    fn append_str(&mut self, string: Escaped<T>) {
        self.inner.push_str(string.inner.as_ref());
    }
}

impl<T: AsRef<str>> Escaped<T> {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

/// A wrapper for writer automatically escaping text written to it
pub struct EscapedWriter<T: Write> {
    inner: T
}

impl<T: Write> EscapedWriter<T> {
    /// Create a new `EscapedWriter`
    pub fn new(inner: T) -> Self {
        EscapedWriter { inner: inner }
    }

    /// Consume the writer, returning the wrapped value
    pub fn into_inner(self) -> T {
        self.inner
    }
}

/// Marks values as escapable
///
/// `Escapable` values can either be escaped by hand
/// or by appending them to either a `SafeWriter` or
/// an `Escaped` value.
///
/// `Escapable` values can also be considered safe
/// by calling the appropriate value. Safe values
/// are exempted from further escaping.
pub trait Escapable: AsRef<str> {
    /// Escape the value at hand and return
    /// an escaped String.
    fn escape(&self) -> Escaped<String>;
    /// Mark the value as safe, exempting it from
    /// further escaping.
    fn safe(&self) -> Escaped<String>;
}

impl<T: AsRef<str>> Escapable for T {
    fn escape(&self) -> Escaped<String> {
        Escaped { inner: encode_attribute(self.as_ref()) }
    }

    fn safe(&self) -> Escaped<String> {
        Escaped { inner: self.as_ref().into() }
    }
}

/// Escaped writing to buffers
///
/// This trait handles writing of different kinds of values
/// to an `EscapedWriter`. It is intended to be implemented
/// for `EscapedWriter` for each kind of value that is allowed
/// to be written to.
///
/// The implementor must properly escape the passed value before
/// writing it.
pub trait EscapedWrite<T> {
    /// Write the passed string-like value to the writer,
    /// returning the writers Result.
    fn write_str(&mut self, value: T) -> io::Result<()>;
}

impl<X: AsRef<str>, W: Write> EscapedWrite<Escaped<X>> for EscapedWriter<W> {
    fn write_str(&mut self, value: Escaped<X>) -> io::Result<()> {
        self.inner.write_all(value.as_ref().as_bytes())
    }
}

impl<'a, X: Escapable, W: Write> EscapedWrite<X> for EscapedWriter<W> {
    fn write_str(&mut self, value: X) -> io::Result<()> {
        encode_attribute_w(value.as_ref(), &mut self.inner)
    }
}


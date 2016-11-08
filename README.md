# escapade - type assisted html safety

`escapade` is inspired by ActiveSupports SafeBuffer.

`escapade` provides String concatenation and writing, but automatically escapes any HTML in the data in the process. This prevents accidental unescaped writes to the output.

The library provides both a String type for HTML-safe concatenation and a writer, wrapping types implementing `Write`.

The library works with any type that implements `AsRef<str>`.

You might want to use this library for your templating language ;).

## Usage

Put the following in the `dependencies` section of your `Cargo.toml`:

```
escapade = "0.0.2"
```

### Writer mode

Use the `SafeWriter` struct to make any type implementing `Write` html safe.

```rust
extern crate escapade;

use escapade::EscapedWriter;
use escapade::EscapedWrite;

fn main() {
    let mut buffer = EscapedWriter::new(vec![]);

    buffer.write_str("<hello>&world</hello>").expect("write should not fail");
    assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", String::from_utf8(buffer.into_inner()).unwrap());
}
```

### String mode

Appending an unescaped string to any escaped string will escape the second string.

```rust
extern crate escapade;

use escapade::Append;
use escapade::Escapable;

fn main() {
    let mut s = String::from("<hello>").escape();
    s.append_str(String::from("&world</hello>"));

    assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", s.into_inner());
}
```

Escaped strings cannot be appended to normal strings.

### Opting into safety

Sometimes, you are sure that the string in question is safe (e.g., you painstakenly created it by hand). You can opt into safety in this case, to avoid escaping:

```rust
extern crate escapade;

use escapade::Escapable;
use escapade::EscapedWriter;
use escapade::EscapedWrite;

fn main() {
    let mut buffer = EscapedWriter::new(vec![]);

    buffer.write_str("<hello>&world</hello>".safe()).expect("write should not fail");
    assert_eq!("<hello>&world</hello>", String::from_utf8(buffer.into_inner()).unwrap());
}
```

## TODO

* Think about the interface a bit more
* Bump to 1.0.0

## License

MIT

## Acknowledgements

Encoding functionality taken from [rust-htmlescape](http://github.com/veddan/rust-htmlescape), by [Viktor Dahl](https://github.com/veddan), licensed under MIT.

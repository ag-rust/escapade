use std::io::{self, Write};
use std::char;
use entities::*;

///
/// HTML entity-encodes a string for use in attributes values.
///
/// Entity-encodes a string using an extensive set of entities, giving a string suitable for use
/// in HTML attribute values. All entities from `encode_minimal` are used, and further, all
/// non-alphanumeric ASCII characters are hex-encoded (`&#x__;`).
/// See the [OWASP XSS Prevention Cheat Sheet](
/// https://www.owasp.org/index.php/XSS_(Cross_Site_Scripting)_Prevention_Cheat_Sheet) for more
/// information on entity-encoding for attribute values.
///
/// # Arguments
/// - `s` - The string to encode.
///
/// # Return value
/// The encoded string.
///
pub fn encode_attribute(s: &str) -> String {
    let mut writer = Vec::with_capacity(s.len() * 3);
    match encode_attribute_w(s, &mut writer) {
        Err(_) => panic!(),
        Ok(_) => String::from_utf8(writer).unwrap()
    }
}

///
/// HTML entity-encodes a string, for use in attributes values, to a writer.
///
/// Similar to `encode_attribute`, except that the output is written to a writer rather
/// than returned as a `String`.
///
/// # Arguments
/// - `s` - The string to encode.
/// - `writer` - Output is written to here.
pub fn encode_attribute_w<W: Write>(s: &str, writer: &mut W) -> io::Result<()> {
    let mut marker = 0;
    let mut flushed = false;

    for (pos, c) in s.char_indices() {
        if flushed {
            marker = pos;
            flushed = false;
        }

        let b = c as usize;

        match get_entity(c) {
            Some(entity) => {
                try!(flush(s, marker, pos, writer));
                flushed = true;
                try!(writer.write_all(entity.as_bytes()))
            }
            None =>
                if b < 256 && (b > 127 || !is_ascii_alnum(c)) {
                    try!(flush(s, marker, pos, writer));
                    flushed = true;
                    try!(write_hex(writer, c))
                }
        };
    }
    if !flushed {
        try!(writer.write_all(&s[marker..].as_bytes()));
    }
    Ok(())
}

fn flush<W: Write>(s: &str, marker: usize, pos: usize, writer: &mut W) -> io::Result<()> {
    let slice = &s[marker..pos];
    writer.write_all(slice.as_bytes())
}

fn get_entity(c: char) -> Option<&'static str> {
    match MINIMAL_ENTITIES.binary_search_by(|&(ec, _)| ec.cmp(&c) ) {
        Err(..) => None,
        Ok(idx) => {
            let (_, e) = MINIMAL_ENTITIES[idx];
            Some(e)
        }
    }
}

fn write_hex<W: Write>(writer: &mut W, c: char) -> io::Result<()> {
    let hex = b"0123456789ABCDEF";
    try!(writer.write(b"&#x"));
    let n = c as u8;
    let bytes = [hex[((n & 0xF0) >> 4) as usize],
                 hex[(n & 0x0F) as usize],
                 b';'];
    writer.write_all(&bytes)
}

fn is_ascii_alnum(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9')
}

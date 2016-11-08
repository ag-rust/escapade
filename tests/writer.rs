extern crate escapade;

#[allow(unused_must_use)]
mod test {
    use escapade::Escapable;
    use escapade::EscapedWriter;
    use escapade::EscapedWrite;

    #[test]
    fn escape_unsafe_string() {
        let mut buffer = EscapedWriter::new(vec![]);

        buffer.write_str(String::from("<hello>&world</hello>"));
        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", String::from_utf8(buffer.into_inner()).unwrap());
    }

    #[test]
    fn escape_unsafe_str() {
        let mut buffer = EscapedWriter::new(vec![]);

        buffer.write_str("<hello>&world</hello>");
        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", String::from_utf8(buffer.into_inner()).unwrap());
    }

    #[test]
    fn escape_safe_str() {
        let mut buffer = EscapedWriter::new(vec![]);
        let s = "<hello>&world</hello>".escape();

        buffer.write_str(s);
        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", String::from_utf8(buffer.into_inner()).unwrap());
    }

    #[test]
    fn mark_string_safe() {
        let mut buffer = EscapedWriter::new(vec![]);

        buffer.write_str("<hello>&world</hello>".safe());
        assert_eq!("<hello>&world</hello>", String::from_utf8(buffer.into_inner()).unwrap());
    }
}

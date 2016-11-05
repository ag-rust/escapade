extern crate safe_buffer;

#[allow(unused_must_use)]
mod test {
    use safe_buffer::Escapable;
    use safe_buffer::Append;

    #[test]
    fn concatenate_safe_and_unsafe() {
        let s = String::from("<hello>").escape();
        let res = s.append_str(String::from("&world</hello>"));

        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", res.into_inner());
    }

    #[test]
    fn concatenate_safe_and_unsafe_str() {
        let s = String::from("<hello>").escape();
        let res = s.append_str("&world</hello>");

        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", res.into_inner());
    }

    #[test]
    fn concatenate_safe_and_safe() {
        let s = String::from("<hello>").safe();
        let res = s.append_str(String::from("&world</hello>").safe());

        assert_eq!("<hello>&world</hello>", res.into_inner());
    }

}

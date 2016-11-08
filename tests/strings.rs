extern crate escapade;

#[allow(unused_must_use)]
mod test {
    use escapade::Escapable;
    use escapade::Append;

    #[test]
    fn concatenate_safe_and_unsafe() {
        let mut s = String::from("<hello>").escape();
        s.append_str(String::from("&world</hello>"));

        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", s.into_inner());
    }

    #[test]
    fn concatenate_safe_and_unsafe_str() {
        let mut s = String::from("<hello>").escape();
        s.append_str("&world</hello>");

        assert_eq!("&lt;hello&gt;&amp;world&lt;&#x2F;hello&gt;", s.into_inner());
    }

    #[test]
    fn concatenate_safe_and_safe() {
        let mut s = String::from("<hello>").safe();
        s.append_str(String::from("&world</hello>").safe());

        assert_eq!("<hello>&world</hello>", s.into_inner());
    }

}

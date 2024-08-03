pub trait Accented {
    fn unaccented(&self) -> String;
}

impl Accented for String {
    fn unaccented(&self) -> String {
        self
            .replace("\u{301}", "")
    }
}

#[cfg(test)]
#[test]
fn remove_accents() {
    assert_eq!( String::from("foo\u{301}").unaccented(), "foo" );
}
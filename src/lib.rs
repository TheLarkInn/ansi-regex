extern crate regex;
use regex::Regex;

pub fn ansi_regex() -> Regex {
    Regex::new(r"\x1b\[([\x30-\x3f]*[\x20-\x2f]*[\x40-\x7e])").unwrap()
}



#[test]
fn match_ansi_code_in_a_string() {
    assert!(ansi_regex().find("\x1b[31mHello\x1b[39m").unwrap().as_str() == "\x1b[31m");
}
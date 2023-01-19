extern crate regex;
use regex::Regex;

pub fn ansi_regex() -> Regex {
    // TODO Can we support all the other sequences?
    // See: chalk/ansi-regex
    Regex::new(r"\x1b\[([\x30-\x3f]*[\x20-\x2f]*[\x40-\x7e])").unwrap()
}

#[test]
fn match_ansi_code_in_a_string() {
    assert!(ansi_regex().is_match("\x1b[31mHello World\x1b[39m"));
    assert!(ansi_regex().is_match("foo\u{001B}[4mcake\u{001B}[0m"));
    assert!(ansi_regex().is_match("\u{001B}[4mcake\u{001B}[0m"));
    assert!(ansi_regex().is_match("\u{001B}[0m\u{001B}[4m\u{001B}[42m\u{001B}[31mfoo\u{001B}[39m\u{001B}[49m\u{001B}[24mfoo\u{001B}[0m"));
    assert!(ansi_regex().is_match("foo\u{001B}[mfoo"));
}

#[test]
fn match_ansi_code_from_ls_command() {
    assert!(ansi_regex().is_match("\u{001B}[00;38;5;244m\u{001B}[m\u{001B}[00;38;5;33mfoo\u{001B}[0m"));
}

#[test]
fn match_reset_setfg_setbg_italics_strike_underline_sequence_in_a_string() {
    let test_string = "\u{001B}[0;33;49;3;9;4mbar\u{001B}[0m";
    let match_string= "\u{001B}[0;33;49;3;9;4m";

    assert!(ansi_regex().is_match(&test_string));
    assert!(ansi_regex().find(&test_string).unwrap().as_str() == match_string)
}

#[test]
fn match_clear_tabs_sequence_in_a_string() {
    let test_string="foo\u{001B}[0gbar";
    let match_string= "\u{001B}[0g";

    assert!(ansi_regex().is_match(&test_string));
    assert!(ansi_regex().find(&test_string).unwrap().as_str() == match_string)
}

#[test]
fn match_clear_line_from_cursor_right_in_a_string() {
    let test_string="foo\u{001B}[Kbar";
    let match_string= "\u{001B}[K";

    assert!(ansi_regex().is_match(&test_string));
    assert!(ansi_regex().find(&test_string).unwrap().as_str() == match_string)
}

#[test]
fn match_clear_screen_in_a_string() {
    let test_string="foo\u{001B}[2Jbar";
    let match_string= "\u{001B}[2J";

    assert!(ansi_regex().is_match(&test_string));
    assert!(ansi_regex().find(&test_string).unwrap().as_str() == match_string)  
}
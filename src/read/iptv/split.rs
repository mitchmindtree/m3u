/// Splits ' key="value"' to (key,value) or None if malformed
pub fn split_key_val(keyval: &str) -> Option<(&str, &str)> {
    let mut parts = keyval.split('=');
    let key = parts.next()?.trim();
    let val: &str = parts.next()?;
    if val.len() < 2 {
        return None;
    }
    let val = &val[1..val.len() - 1];
    Some((key, val))
}

#[test]
fn test_split_key_val() {
    assert_eq!(split_key_val(" key=\"value\""), Some(("key", "value")));
    assert_eq!(split_key_val("key=\""), None);
    assert_eq!(split_key_val(""), None);
    assert_eq!(split_key_val("="), None);
    assert_eq!(split_key_val("=\"toto\""), Some(("", "toto")));
    assert_eq!(
        split_key_val(r#"key="value "1"""#),
        Some(("key", "value \"1\""))
    );
}

pub fn next_closing_mark_index(s: &str, offset: usize) -> Option<usize> {
    let s = &s[offset..];
    if s.starts_with(",") {
        return None;
    }

    s.char_indices()
        .filter(|&(_, c)| c == '"')
        .skip(1)
        .step_by(2)
        .map(|(i, _)| (i, is_closing_mark(s, i)))
        .skip_while(|(_, role)| *role == QuotationMarkRole::INNER)
        .map(|(i, _)| i + offset)
        .next()
}

#[test]
fn test_next_closing_mark() {
    let s = r#"a="3" b="9 "13"" c="21"24"27",31, "36""#;
    //         01234567891113151719212325272931333537
    assert_eq!(next_closing_mark_index(s, 0), Some(4));
    assert_eq!(next_closing_mark_index(s, 5), Some(15));
    assert_eq!(next_closing_mark_index(s, 16), Some(28));
    assert_eq!(next_closing_mark_index(s, 29), None);
}

#[derive(PartialEq, Debug)]
enum QuotationMarkRole {
    CLOSING,
    INNER,
    ENDING,
}

/// ENDING if '"' is followed by /,/
/// CLOSING if " is followed by /\s*$| +[\w]+="/
/// INNER elsewhere
fn is_closing_mark(s: &str, offset: usize) -> QuotationMarkRole {
    if offset == s.trim_end().len() - 1 {
        return QuotationMarkRole::CLOSING;
    }
    let mut chars = s[offset + 1..].chars();
    let next_char = chars.next();
    if next_char == Some(',') {
        return QuotationMarkRole::ENDING;
    }
    if next_char != Some(' ') {
        return QuotationMarkRole::INNER;
    }
    let mut chars = chars.skip_while(|c: &char| *c == ' ');
    if chars.next().map(char::is_alphabetic) != Some(true) {
        return QuotationMarkRole::INNER;
    }
    let mut chars = chars.skip_while(|c: &char| char::is_alphabetic(*c) || *c == '-');
    if chars.next() == Some('=') && chars.next() == Some('"') {
        QuotationMarkRole::CLOSING
    } else {
        QuotationMarkRole::INNER
    }
}

#[test]
fn test_is_closing_mark() {
    fn check(s: &str, i: usize, expected: QuotationMarkRole) {
        let actual = is_closing_mark(s, i);
        if actual != expected {
            panic!("i={}, expected={:?}, s={}", i, expected, &s[i..]);
        }
    }
    let s1 = r#"keya="val1" keyb="val 2" keyc="val "3"" keyd="""#;
    use self::QuotationMarkRole::*;
    check(s1, 5, INNER);
    check(s1, 10, CLOSING);
    check(s1, 17, INNER);
    check(s1, 23, CLOSING);
    check(s1, 30, INNER);
    check(s1, 35, INNER);
    check(s1, 37, INNER);
    check(s1, 38, CLOSING);
    check(s1, 45, INNER);
    check(s1, 46, CLOSING);
    let s2 = r#"tvg-name="PPV- WRESTLING 03 | GCW: "Bring Em Out" (09.06-4:30PM ET)""#;
    check(s2, 10, INNER);
    check(s2, 35, INNER);
    check(s2, 48, INNER);
    check(s2, 57, INNER);
    let s3 = r#" keya="val1"  keyb="val "2"""#;
    check(s3, 6, INNER);
    check(s3, 11, CLOSING);
    check(s3, 19, INNER);
    check(s3, 24, INNER);
    check(s3, 26, INNER);
    check(s3, 27, CLOSING);
    let s4 = r#"a="3" b="9 "13"" c="21"24"27",31, "36""#;
    //          01234567891113151719212325272931333537
    check(s4, 2, INNER);
    check(s4, 4, CLOSING);
    check(s4, 8, INNER);
    check(s4, 11, INNER);
    check(s4, 14, INNER);
    check(s4, 15, CLOSING);
    check(s4, 19, INNER);
    check(s4, 22, INNER);
    check(s4, 25, INNER);
    check(s4, 28, ENDING);
}

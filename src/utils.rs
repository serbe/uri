use crate::error::{Error, Result};

use percent_encoding::percent_decode_str;

// reserved    = gen-delims / sub-delims
// gen-delims  = ":" / "/" / "?" / "#" / "[" / "]" / "@"
// sub-delims  = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
// unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"

// pub const GEN_DELIMS: [char; 7] = [':', '/', '?', '#', '[', ']', '@'];
pub(crate) const SUB_DELIMS: [char; 11] = ['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '='];
pub(crate) const UNRESERVED: [char; 4] = ['-', '.', '_', '~'];
pub(crate) const SCHEME_ALLOWED_CHARS: [char; 3] = ['+', '-', '.'];

/// userinfo    = *( unreserved / pct-encoded / sub-delims / ":" )
pub(crate) fn is_valid_ups(input: &str, colon: bool) -> bool {
    let additionals = if colon { vec![':', '%'] } else { vec!['%'] };
    input.chars().all(|ch| {
        ch.is_alphanumeric()
            || UNRESERVED.contains(&ch)
            || SUB_DELIMS.contains(&ch)
            || additionals.contains(&ch)
    })
}

pub(crate) fn check_ups(input: &str, colon: bool, err: Error) -> Result<()> {
    match is_valid_ups(input, colon) {
        true => Ok(()),
        false => Err(err),
    }
}

/// scheme      = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
pub(crate) fn is_valid_scheme(input: &str) -> bool {
    input[0..1].chars().all(|ch| ch.is_alphabetic())
        && input[1..]
            .chars()
            .all(|ch| ch.is_alphanumeric() || SCHEME_ALLOWED_CHARS.contains(&ch))
}

pub(crate) fn decode(v: &str) -> Option<String> {
    percent_decode_str(v)
        .decode_utf8()
        .map_or(None, |op| Some(op.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn userinfo_with_colon() {
        let s = "asd1209!$&,:()*+,;=-._~";
        assert!(is_valid_ups(s, true));
    }

    #[test]
    fn scheme() {
        let s = "asd1209+-.";
        assert!(is_valid_scheme(s));
    }

    #[test]
    fn reserver_char() {
        let bad_str = "myscheme://authority<\"hi\">/foo";
        assert!(!is_valid_ups(bad_str, false));
        let good_str = "myschemeauthority!$&()*:+,;=-._~";
        assert!(is_valid_ups(good_str, true));
    }
}

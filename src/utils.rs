use percent_encoding::percent_decode_str;

// reserved    = gen-delims / sub-delims
// gen-delims  = ":" / "/" / "?" / "#" / "[" / "]" / "@"
// sub-delims  = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
// unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"

pub const GEN_DELIMS: [char; 7] = [':', '/', '?', '#', '[', ']', '@'];
pub const SUB_DELIMS: [char; 11] = ['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '='];
pub const UNRESERVED: [char; 4] = ['-', '.', '_', '~'];
pub const SCHEME_ALLOWED_CHARS: [char; 3] = ['+' , '-' , '.'];

/// userinfo    = *( unreserved / pct-encoded / sub-delims / ":" )
pub fn is_valid_userinfo(input: &str) -> bool {
    input.chars().all(|ch| ch.is_alphanumeric() || UNRESERVED.contains(&ch) || SUB_DELIMS.contains(&ch) || ch==':' || ch =='%')
}

/// scheme      = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
pub fn is_valid_scheme(input: &str) -> bool {
    input[0..1].chars().all(|ch| ch.is_alphabetic())
        && input[1..]
            .chars()
            .all(|ch| ch.is_alphanumeric() || SCHEME_ALLOWED_CHARS.contains(&ch))
}

pub fn decode(v: &str) -> Option<String> {
    percent_decode_str(v)
        .decode_utf8()
        .map_or(None, |op| Some(op.to_string()))
}

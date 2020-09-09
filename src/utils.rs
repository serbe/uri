use percent_encoding::percent_decode_str;

// reserved    = gen-delims / sub-delims
// gen-delims  = ":" / "/" / "?" / "#" / "[" / "]" / "@"
// sub-delims  = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
// unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"

// scheme      = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
// userinfo    = *( unreserved / pct-encoded / sub-delims / ":" )

pub const GEN_DELIMS: [char; 7] = [':', '/', '?', '#', '[', ']', '@'];
pub const SUB_DELIMS: [char; 11] = ['!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '='];
pub const UNRESERVED: [char; 4] = ['-', '.', '_', '~'];

pub fn contains_gen_delims(input: &str) -> bool {
    input.chars().any(|ch| GEN_DELIMS.contains(&ch))
}

pub fn contains_sub_delims(input: &str) -> bool {
    input.chars().any(|ch| SUB_DELIMS.contains(&ch))
}

pub fn all_unreserved(input: &str) -> bool {
    input
        .chars()
        .any(|ch| ch.is_alphanumeric() || UNRESERVED.contains(&ch))
}

pub(crate) fn decode(v: &str) -> Option<String> {
    percent_decode_str(v)
        .decode_utf8()
        .map_or(None, |op| Some(op.to_string()))
}

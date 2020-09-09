/*!
 * rust-uri is is an implementation of the Uniform Resource Identifier (URI)
 *
 * # URI parsing and data structures
 *
 * https://tools.ietf.org/html/rfc3986
 * https://www.protocols.ru/WP/rfc3986
 *
 */

pub use crate::uri::Uri;
pub use addr::Addr;
pub use authority::Authority;
pub use error::{Error, Result};
pub use range::RangeUsize;

mod addr;
mod authority;
mod error;
mod range;
mod uri;
mod utils;

// pct-encoded = "%" HEXDIG HEXDIG

// reserved    = gen-delims / sub-delims
// gen-delims  = ":" / "/" / "?" / "#" / "[" / "]" / "@"
// sub-delims  = "!" / "$" / "&" / "'" / "(" / ")" / "*" / "+" / "," / ";" / "="
// unreserved  = ALPHA / DIGIT / "-" / "." / "_" / "~"

// scheme      = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
// userinfo    = *( unreserved / pct-encoded / sub-delims / ":" )

//
// For each URI reference (R), the following pseudocode describes an
//    algorithm for transforming R into its target URI (T):

//       -- The URI reference is parsed into the five URI components
//       --
//       (R.scheme, R.authority, R.path, R.query, R.fragment) = parse(R);

//       -- A non-strict parser may ignore a scheme in the reference
//       -- if it is identical to the base URI's scheme.
//       --
//       if ((not strict) and (R.scheme == Base.scheme)) then
//          undefine(R.scheme);
//       endif;

//       if defined(R.scheme) then
//          T.scheme    = R.scheme;
//          T.authority = R.authority;
//          T.path      = remove_dot_segments(R.path);
//          T.query     = R.query;
//       else
//          if defined(R.authority) then
//             T.authority = R.authority;
//             T.path      = remove_dot_segments(R.path);
//             T.query     = R.query;
//          else
//             if (R.path == "") then
//                T.path = Base.path;
//                if defined(R.query) then
//                   T.query = R.query;
//                else
//                   T.query = Base.query;
//                endif;
//             else
//                if (R.path starts-with "/") then
//                   T.path = remove_dot_segments(R.path);
//                else
//                   T.path = merge(Base.path, R.path);
//                   T.path = remove_dot_segments(T.path);
//                endif;
//                T.query = R.query;
//             endif;
//             T.authority = Base.authority;
//          endif;
//          T.scheme = Base.scheme;
//       endif;

//       T.fragment = R.fragment;

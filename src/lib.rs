/*!
 * rust-uri is is an implementation of the Uniform Resource Identifier (URI)
 * 
 * # URI parsing and data structures
 * 
 * 
 * 
 */

pub use crate::uri::Uri;
pub use addr::Addr;
pub use authority::Authority;
pub use error::{Error, Result};

mod addr;
mod authority;
mod error;
mod range;
mod uri;

// use std::ops::{Range, RangeFrom, RangeTo};
use std::str::FromStr;

use crate::error::{Error, Result};

// mod addr;
mod error;
// mod range;
// mod authority;
// mod uri;

#[derive(Debug, Clone, PartialEq)]
pub struct Uri {
    inner: String,
    scheme_end: usize,
    authority_end: Option<usize>,
    path_start: Option<usize>,
    query_start: Option<usize>,
    fragment_start: Option<usize>,
}

impl FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let inner = s.to_string();
        //         // remove_spaces(&mut s);
        let scheme_end = get_scheme(s)?;
        let uri_part = s.split_at(scheme_end);

        let fragment_start = uri_part.1.find('#');

        let uri_part = if let Some(pos) = fragment_start {
            uri_part.1.split_at(pos).0
        } else {
            uri_part.1
        };

        let query_start = uri_part.find('?');

        let uri_part = if let Some(pos) = query_start {
            uri_part.split_at(pos).0
        } else {
            uri_part
        };

        let authority_end = if uri_part.starts_with("//") {
            get_authority_end(s)
        } else {
            Ok(None)
        }?;

        let uri_part = if let Some(pos) = authority_end {
            uri_part.split_at(pos).0
        } else {
            uri_part
        };

        let path_start = uri_part.find('/');

        // let uri_part = if let Some(pos) = query_start {
        //     uri_part.split_at(pos).0
        // } else {
        //     uri_part
        // };

        //         let (scheme, maybe_part) = get_chunks(&s, uri_part, ":", true, false);
        //         let (scheme, mut uri_part) = if let Some(scheme) = scheme {
        //             let range = Range::from(scheme);
        //             if s[range.clone()].chars().all(|c| c.is_alphanumeric()) {
        //                 s.replace_range(range.clone(), &s[range].to_lowercase());
        //                 (scheme, maybe_part)
        //             } else {
        //                 return Err(Error::EmptyScheme);
        //             }
        //         } else {
        //             return Err(Error::EmptyScheme);
        //         };

        //         let authority = if let Some(u) = &uri_part {
        //             let (auth, part) = if s[*u].contains("//") {
        //                 get_chunks(
        //                     &s,
        //                     Some(RangeUsize::new(u.start + 2, u.end)),
        //                     "/",
        //                     false,
        //                     false,
        //                 )
        //             } else {
        //                 get_chunks(&s, uri_part, "/", false, false)
        //             };
        //             if let Some(a) = auth {
        //                 uri_part = part;
        //                 s[a].parse::<Authority>()?
        //             } else {
        //                 return Err(Error::EmptyAuthority);
        //             }
        //         } else {
        //             return Err(Error::EmptyAuthority);
        //         };

        //         let addr = authority.host().parse::<Addr>()?;

        //         let (path, _uri_part) = get_chunks(&s, uri_part, "?", false, false);

        Ok(Uri {
            inner,
            scheme_end,
            authority_end,
            // addr,
            path_start,
            query_start,
            fragment_start,
        })
    }
}

// fn get_range(s: &str, ch: char) {
//     println!("{:?}", s.contains(ch));
//     println!("{:?}", s.find(ch));
// }

// fn get_path_start(s: &str) -> Option<usize> {
//    s.find('/')
// }

fn contain_reserver_char(s: &str) -> bool {
    s.chars()
        .any(|ch| [':', '/', '?', '#', '[', ']', '@'].contains(&ch))
}

// fn check_host(s: &str) -> Result<()> {
//     Ok(())
// }

fn check_user_info(s: &str) -> Result<()> {
    if let Some(colon_pos) = s.find(':') {
        let user_info = s.split_at(colon_pos);
        if user_info.0.is_empty() {
            Err(Error::EmptyUsername)
        } else if contain_reserver_char(user_info.0) {
            Err(Error::InvalidUsername(user_info.0.to_string()))
        } else if contain_reserver_char(user_info.1) {
            Err(Error::InvalidPassword(user_info.1.to_string()))
        } else {
            Ok(())
        }
    } else {
        if s.is_empty() {
            Err(Error::EmptyUsername)
        } else if contain_reserver_char(s) {
            Err(Error::InvalidUsername(s.to_string()))
        } else {
            Ok(())
        }
    }?;
    Ok(())
}

fn get_user_info_end(s: &str) -> Result<Option<usize>> {
    if let Some(pos) = s.find('@') {
        let part = s.split_at(pos);
        let _ = check_user_info(part.0)?;
        Ok(Some(pos))
    } else {
        Ok(None)
    }
}

fn get_host_end(s: &str) -> Result<usize> {
    if s.is_empty() {
        Err(Error::EmptyHost)
    } else {
        let split_at = if s.starts_with('[') && s.contains(']') {
            "]:"
        } else {
            ":"
        };
        Ok(if let Some(pos) = s.rfind(split_at) {
            pos
        } else {
            s.len()
        })
    }
}

fn check_port(s: &str) -> Result<()> {
    s.parse::<u16>()
        .map(|_| ())
        .map_err(|_| Error::ParsePort(s.to_string()))
}

fn get_authority_end(s: &str) -> Result<Option<usize>> {
    let mut len = 0;
    let user_info_end = get_user_info_end(s)?;

    let part = if let Some(pos) = user_info_end {
        len = len + pos + 1;
        s.split_at(pos).1
    } else {
        s
    };

    let part = if let Some(pos) = part.find('/') {
        part.split_at(pos).0
    } else {
        part
    };

    let host_end = get_host_end(part)?;

    len = if host_end < part.len() {
        let _ = check_port(part.split_at(host_end).1)?;
        len + part.len()
    } else {
        len + host_end
    };

    if len > 0 {
        Ok(Some(len))
    } else {
        Ok(None)
    }
}

fn check_scheme(scheme: &str) -> Result<()> {
    if scheme.is_empty() {
        return Err(Error::EmptyScheme);
    }
    if scheme.char_indices().all(|(pos, ch)| {
        if pos == 0 {
            ch.is_alphabetic()
        } else {
            ch.is_alphanumeric() || ch == '+' || ch == '.' || ch == '-'
        }
    }) {
        Ok(())
    } else {
        Err(Error::InvalidScheme(scheme.to_string()))
    }
}

fn get_scheme(s: &str) -> Result<usize> {
    if let Some(pos) = s.find(':') {
        let (scheme, _) = s.split_at(pos);
        check_scheme(scheme)?;
        Ok(pos)
    } else {
        Err(Error::EmptyScheme)
    }
}

fn main() -> Result<()> {
    let s = "asd://webmaster@www.example.org/";
    let uri = s.parse::<Uri>();
    println!("{:?}", uri);
    Ok(())
}
